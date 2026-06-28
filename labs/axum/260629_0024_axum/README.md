# 260629_0024 — Axum ↔ Express Cross-Talk Example

## Build & Run

Use **two terminals**. Start Axum first, then Express.

### Terminal 1 — Axum (`examples/axum/260629_0024_axum`)

```bash
cd examples/axum/260629_0024_axum

# Dev (compile + run)
PEER_URL=http://127.0.0.1:8202 cargo run

# Release build only
cargo build --release

# Run release binary
PEER_URL=http://127.0.0.1:8202 ./target/release/example-axum
```

### Terminal 2 — Express (`examples/express/260629_0024_express`)

```bash
cd examples/express/260629_0024_express

pnpm install --ignore-workspace
PEER_URL=http://127.0.0.1:8201 pnpm start
```

Both servers must be running for `/relay` and `/demo/roundtrip` to work.  
Press **Ctrl+C** in each terminal to stop.

| Server  | Default port | Peer URL env        |
|---------|--------------|---------------------|
| Axum    | 8201         | `PEER_URL=http://127.0.0.1:8202` |
| Express | 8202         | `PEER_URL=http://127.0.0.1:8201` |

---

Two standalone web servers that exchange data over **async localhost HTTP**:

| Path | Stack | Port |
|------|-------|------|
| `examples/axum/260629_0024_axum` | Rust + Axum + reqwest | 8201 |
| `examples/express/260629_0024_express` | Node + Express + fetch | 8202 |

Each server is both an HTTP **server** and an HTTP **client**. They call each other's `/ingest` endpoint, run relay demos, and send background heartbeats every 5 seconds.

---

## Directory Layout

```
examples/
├── axum/
│   └── 260629_0024_axum/          ← Axum server (this README)
│       ├── Cargo.toml
│       ├── README.md
│       └── src/main.rs
└── express/
    └── 260629_0024_express/       ← Express server (paired)
        ├── package.json
        ├── tsconfig.json
        └── src/
            ├── index.ts           # HTTP server boot
            ├── app.ts             # Routes
            ├── peer.ts            # HTTP client + heartbeat
            └── types.ts           # Shared types
```

---

## Architecture

```
examples/axum/260629_0024_axum          :8201
     │  reqwest POST /ingest
     └──────────────────────────────►  examples/express/260629_0024_express
                                             :8202
     ◄──────────────────────────────  fetch POST /ingest

Background: both send heartbeat to peer /ingest every 5s
```

Both bind to **`127.0.0.1` only**.

---

## Communication Patterns

### Relay — `POST /relay`

1. Store message locally.
2. Async HTTP `POST` to peer `/ingest`.
3. Return `{ local, peer }`.

### Ingest — `POST /ingest`

Inbound endpoint for the peer. Stores the message and returns `{ ok, stored, reply }`.  
Reply text is reversed and prefixed with `ack from <service>`.

### Round-trip demo — `GET /demo/roundtrip`

Automated 4-step exchange:

| Step | Actor | Action |
|------|-------|--------|
| 1 | Caller | `POST /relay` to peer |
| 2 | Peer | Responds via `/ingest` |
| 3 | Caller | Triggers peer `POST /relay` |
| 4 | Peer | Relays back via `/ingest` |

Callable on either server (`:8201` or `:8202`).

### Heartbeat

- **Axum:** `tokio::spawn` + 5s interval → `reqwest` POST `/ingest`
- **Express:** `setInterval(5000)` → `fetch` POST `/ingest`

Count visible at `GET /health` → `heartbeats_sent`.

---

## API Reference

Both servers expose the same routes.

### `GET /health`

```json
{
  "ok": true,
  "service": "axum",
  "peer_url": "http://127.0.0.1:8202",
  "message_count": 3,
  "heartbeats_sent": 12
}
```

### `GET /messages`

Array of all stored messages (ingests, relays, heartbeats).

### `POST /ingest`

```json
// Request
{ "from": "axum", "text": "hello", "id": "optional", "timestamp": 1719661440000 }

// Response
{
  "ok": true,
  "stored": { "id": "...", "from": "axum", "text": "hello", "timestamp": 1719661440000 },
  "reply": { "id": "...", "from": "express", "text": "ack from express: olleh", "timestamp": 1719661440001 }
}
```

### `POST /relay`

```json
// Request
{ "text": "hello from client" }

// Response
{
  "local": { "id": "...", "from": "axum", "text": "hello from client", "timestamp": 1719661440000 },
  "peer": { "ok": true, "stored": { "..." }, "reply": { "..." } }
}
```

Returns **502** if peer is unreachable.

### `GET /demo/roundtrip`

Returns `{ "steps": [ ... ] }` with the full automated exchange log.

---

## Environment Variables

| Variable | Default | Used by |
|----------|---------|---------|
| `EXAMPLE_AXUM_PORT` | `8201` | Axum |
| `EXAMPLE_EXPRESS_PORT` | `8202` | Express |
| `PORT` | same as above | Either (fallback) |
| `PEER_URL` | `http://127.0.0.1:8202` (Axum) / `http://127.0.0.1:8201` (Express) | Both |

Example with custom ports:

```bash
# Terminal 1
EXAMPLE_AXUM_PORT=9201 PEER_URL=http://127.0.0.1:9202 cargo run

# Terminal 2
EXAMPLE_EXPRESS_PORT=9202 PEER_URL=http://127.0.0.1:9201 pnpm start
```

---

## Verify

After both servers are up:

```bash
curl -s http://127.0.0.1:8201/health | jq
curl -s http://127.0.0.1:8202/health | jq

curl -s -X POST http://127.0.0.1:8201/relay \
  -H 'Content-Type: application/json' \
  -d '{"text":"hello from axum"}' | jq

curl -s http://127.0.0.1:8202/demo/roundtrip | jq

curl -s http://127.0.0.1:8201/messages | jq
curl -s http://127.0.0.1:8202/messages | jq
```

Wait ~5s and re-check `/health` — `heartbeats_sent` should increase on both sides.

---

## Implementation Notes

### Axum — `src/main.rs`

| Concern | Choice |
|---------|--------|
| HTTP server | Axum 0.8 on Tokio |
| HTTP client | reqwest 0.12 (`rustls-tls`) |
| State | `Arc<AppState>` + `tokio::sync::Mutex<Vec<Message>>` |
| Shutdown | Ctrl+C / SIGTERM |

### Express — `src/peer.ts`, `app.ts`, `index.ts`

| Concern | Choice |
|---------|--------|
| HTTP server | Express 4 |
| HTTP client | Native `fetch` (Node 18+) |
| State | In-memory `messages[]` |
| Runner | `tsx` (no build step required for dev) |

> Express folder is outside the root pnpm workspace — always run `pnpm install --ignore-workspace` inside `examples/express/260629_0024_express`.

---

## Prerequisites

| Tool | Version | Used by |
|------|---------|---------|
| Rust + Cargo | stable | Axum |
| Node.js | 18+ | Express (`fetch`) |
| pnpm | any recent | Express deps |

---

## Quick Reference

| Item | Value |
|------|-------|
| Axum path | `examples/axum/260629_0024_axum/` |
| Express path | `examples/express/260629_0024_express/` |
| Axum port | 8201 |
| Express port | 8202 |
| Axum run | `PEER_URL=http://127.0.0.1:8202 cargo run` |
| Express run | `PEER_URL=http://127.0.0.1:8201 pnpm start` |
