import { Hono } from "hono";
import { cors } from "hono/cors";

interface Env {
  KV: KVNamespace;
  R2: R2Bucket;
}

interface UserRecord {
  pubkey: string;
  encrypted_privkey: string;
  passphrase_hash: string;
  ip: string;
  created_at: number;
}

interface Bottle {
  id: string;
  from: string;
  to: string;
  body: string;
  sender_pubkey?: string;
  encrypted: boolean;
  timestamp: number;
}

const app = new Hono<{ Bindings: Env }>();
app.use("*", cors());

async function sha256hex(s: string): Promise<string> {
  const buf = await crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode(s)
  );
  return Array.from(new Uint8Array(buf))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

async function verifyAuth(
  req: Request,
  env: Env
): Promise<string | null> {
  const auth = req.headers.get("Authorization");
  if (!auth?.startsWith("Basic ")) return null;
  const decoded = atob(auth.slice(6));
  const colon = decoded.indexOf(":");
  if (colon === -1) return null;
  const username = decoded.slice(0, colon);
  const hash = decoded.slice(colon + 1);
  const record = await env.KV.get<UserRecord>(`user:${username}`, "json");
  if (!record || record.passphrase_hash !== hash) return null;
  return username;
}

function getIP(req: Request): string {
  return (
    req.headers.get("CF-Connecting-IP") ||
    req.headers.get("X-Forwarded-For")?.split(",")[0].trim() ||
    "unknown"
  );
}

// POST /register
app.post("/register", async (c) => {
  const { username, passphrase_hash, pubkey, encrypted_privkey } =
    await c.req.json<{
      username: string;
      passphrase_hash: string;
      pubkey: string;
      encrypted_privkey: string;
    }>();

  if (!username || !passphrase_hash || !pubkey || !encrypted_privkey) {
    return c.json({ error: "missing fields" }, 400);
  }
  if (!/^[a-zA-Z0-9_-]{3,32}$/.test(username)) {
    return c.json({ error: "invalid username" }, 400);
  }

  const existing = await c.env.KV.get(`user:${username}`);
  if (existing) return c.json({ error: "username taken" }, 409);

  const ip = getIP(c.req.raw);
  const ipCount = parseInt((await c.env.KV.get(`ip:${ip}`)) || "0");
  if (ipCount >= 5) return c.json({ error: "IP limit reached" }, 429);

  const record: UserRecord = {
    pubkey,
    encrypted_privkey,
    passphrase_hash,
    ip,
    created_at: Date.now(),
  };

  await Promise.all([
    c.env.KV.put(`user:${username}`, JSON.stringify(record)),
    c.env.KV.put(`ip:${ip}`, String(ipCount + 1)),
  ]);

  return c.json({ ok: true });
});

// POST /login
app.post("/login", async (c) => {
  const { username, passphrase_hash } = await c.req.json<{
    username: string;
    passphrase_hash: string;
  }>();
  const record = await c.env.KV.get<UserRecord>(`user:${username}`, "json");
  if (!record || record.passphrase_hash !== passphrase_hash) {
    return c.json({ error: "invalid credentials" }, 401);
  }
  return c.json({
    encrypted_privkey: record.encrypted_privkey,
    pubkey: record.pubkey,
  });
});

// GET /user/:username
app.get("/user/:username", async (c) => {
  const record = await c.env.KV.get<UserRecord>(
    `user:${c.req.param("username")}`,
    "json"
  );
  if (!record) return c.json({ error: "not found" }, 404);
  return c.json({ pubkey: record.pubkey });
});

// POST /throw
app.post("/throw", async (c) => {
  const username = await verifyAuth(c.req.raw, c.env);
  if (!username) return c.json({ error: "unauthorized" }, 401);

  const bottle = await c.req.json<Bottle>();
  if (!bottle.id || !bottle.to || !bottle.body) {
    return c.json({ error: "missing fields" }, 400);
  }

  const bodySize = new TextEncoder().encode(bottle.body).length;
  if (bodySize > 128 * 1024) return c.json({ error: "body too large" }, 413);

  bottle.from = username;
  await c.env.R2.put(
    `bottles/${bottle.to}/${bottle.id}.json`,
    JSON.stringify(bottle),
    { httpMetadata: { contentType: "application/json" } }
  );
  return c.json({ ok: true });
});

// GET /bottles/:username
app.get("/bottles/:username", async (c) => {
  const username = await verifyAuth(c.req.raw, c.env);
  if (!username || username !== c.req.param("username")) {
    return c.json({ error: "unauthorized" }, 401);
  }

  const list = await c.env.R2.list({ prefix: `bottles/${username}/` });
  const metas = await Promise.all(
    list.objects.map(async (obj) => {
      const raw = await c.env.R2.get(obj.key);
      if (!raw) return null;
      const b = await raw.json<Bottle>();
      return { id: b.id, from: b.from, encrypted: b.encrypted, timestamp: b.timestamp };
    })
  );
  return c.json(metas.filter(Boolean));
});

// GET /bottle/:id
app.get("/bottle/:id", async (c) => {
  const username = await verifyAuth(c.req.raw, c.env);
  if (!username) return c.json({ error: "unauthorized" }, 401);

  const id = c.req.param("id");
  const obj = await c.env.R2.get(`bottles/${username}/${id}.json`);
  if (!obj) return c.json({ error: "not found" }, 404);
  return c.json(await obj.json());
});

// DELETE /bottle/:id
app.delete("/bottle/:id", async (c) => {
  const username = await verifyAuth(c.req.raw, c.env);
  if (!username) return c.json({ error: "unauthorized" }, 401);

  const id = c.req.param("id");
  await c.env.R2.delete(`bottles/${username}/${id}.json`);
  return c.json({ ok: true });
});

export default app;
