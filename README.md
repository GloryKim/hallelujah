# Hallelujah

A learning and experimentation monorepo that runs a **Tauri desktop app** alongside **local HTTP sidecars** written in multiple languages.

The React UI talks to sidecars over `127.0.0.1`, and to the Tauri shell over IPC — so you can see how data flows between the WebView, Rust backend, and each web server.

## What’s in here

| Layer | Location | Role |
|-------|----------|------|
| Desktop app | `apps/desktop/` | Tauri + React (Data Flow Monitor UI) |
| Sidecars | `services/*/` | Per-language web server examples |
| Contracts | `packages/contracts/` | Shared port and naming config |
| API clients | `packages/api-client/` | TypeScript clients for calling sidecars from React |

### Current sidecars

| Service | Port | Stack |
|---------|------|-------|
| gin | 7101 | Go + Gin |
| express | 7102 | Node + Express |
| fastapi | 7103 | Python + FastAPI |
| nest | 7104 | NestJS |
| axum | 7105 | Rust + Axum |

Each sidecar implements `GET /health` and `GET /meta`, and binds to `127.0.0.1` only.

## Examples

The `examples/` directory holds **standalone, minimal(?) sample projects** — separate from the monorepo sidecars above. They are grouped by topic (e.g. `tokio/`, `axum/`, `express/`, `etc...`).

More simple(?), focused example code will be added over time. Each entry is meant to illustrate one idea or pattern on its own, without the full Tauri + sidecar wiring.

## Quick start

```bash
pnpm install
pnpm dev
```

`pnpm dev` starts all five sidecars from source, then launches the Tauri app.  
If something fails, see [docs/12_troubleshooting.md](./docs/12_troubleshooting.md).

```bash
pnpm generate         # ports.yaml → Rust/TS codegen
pnpm build:sidecars   # build production sidecar binaries
pnpm build            # sidecars + Tauri desktop bundle
```

## Documentation & patch notes

Design notes, development logs, patch notes, and troubleshooting live under **`docs/`**.

- Entry point: [docs/readme.md](./docs/readme.md)
- Topics include Axum sidecar setup, CORS, dev workflow, API contract, and more (`03_` – `12_`)

## Roadmap

This repo is meant to **keep growing**: we plan to add more web server framework examples and attach them as sidecars to the Tauri app, following the same pattern (`services/<name>/` + `ports.yaml` registration).

## License

Private / learning project.
