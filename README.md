# Bottles - 漂流瓶

A desktop app where you throw message "bottles" to specific recipients across the ocean.

## Features
- Glass-morphism UI with 2 tabs: Throw / My Bottles
- End-to-end encryption (X25519 ECDH + AES-256-GCM)
- Cloudflare Worker backend (KV for users, R2 for messages)
- First message plaintext with public key exchange, then fully encrypted

## Prerequisites

### macOS
```bash
# Xcode Command Line Tools
xcode-select --install

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows
1. Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (select "Desktop development with C++")
2. Install [Rust](https://rustup.rs/)
3. Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (included in Windows 10 1803+)

### All Platforms
```bash
# Node.js (v18+)
# npm dependencies
npm install
```

## Development

```bash
# Start Tauri dev server
npm run tauri dev

# Or start frontend only
npm run dev
```

## Build for Production

```bash
npm run tauri build
```
The built app will be in `src-tauri/target/release/bundle/`.

## Cloudflare Worker Deployment

```bash
cd worker
npm install
# Update wrangler.toml with your KV namespace ID and R2 bucket name
wrangler deploy
```

Set the `WORKER_URL` environment variable or update it in `src-tauri/src/lib.rs` before building.

## Architecture

```
┌──────────┐     HTTPS      ┌──────────────┐
│  Tauri   │ ────────────▶ │  Cloudflare  │
│  Desktop │ ◀──────────── │  Worker      │
│  App     │                │  (Hono)      │
└──────────┘                ├──────┬───────┤
                             │ KV   │  R2  │
                             └──────┴───────┘
```

- **KV**: Usernames, encrypted private keys, public keys, IP registration count
- **R2**: Bottle messages (128KB max)
- **Auth**: Basic auth with SHA-256 passphrase hash
- **Encryption**: X25519 ECDH + HKDF + AES-256-GCM
