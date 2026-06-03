# Bottles

`Bottles` is a tiny desktop ocean for people who want to send secret little notes instead of behaving normally.

You type a username.  
You write a message.  
You throw it into the sea.  
Somewhere else, another goblin opens a bottle and reads it.

That is the whole joke.  
That is also the product.

## What This App Does

- Lets you throw message bottles at a specific person.
- Keeps the first exchange simple so two people can discover each other.
- Encrypts later messages so the ocean stays dramatic, not leaky.
- Looks like a dreamy glass postcard instead of enterprise suffering.

## Why It Exists

Because regular messengers are too efficient.

Sometimes you do not want:

- channels
- threads
- status circles
- read receipts
- “sent from my iPhone”

Sometimes you want:

- mystery
- tides
- emotional risk
- a bottle

## How To Use It

1. Open the app.
2. Log in, or quietly become a new user by accident.
3. Throw a bottle at someone’s username.
4. Open `My Bottles`.
5. Read what washed back.
6. Pretend you live on a poetic coastline.

## Technical Truths, But Only A Few

- Desktop app: `Tauri + Svelte`
- Encryption: `X25519 + HKDF + AES-256-GCM`
- Backend: `Cloudflare Worker`
- Storage: `KV` for people, `R2` for bottles

Everything else is just plumbing wearing a raincoat.

## Running It

```bash
npm install
npm run tauri dev
```

If you want a production build:

```bash
npm run tauri build
```

## Backend Notes

The worker lives in `worker/`.

Deploy it when you are ready for your ocean to become a public service:

```bash
cd worker
npm install
wrangler deploy
```

If the desktop app needs a different backend URL, update `WORKER_URL`.

## Final Warning

This app contains:

- cryptography
- feelings
- usernames
- drifting glass

Use responsibly.
