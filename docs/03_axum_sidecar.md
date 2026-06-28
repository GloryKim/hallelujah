# Axum Sidecar — Rust HTTP Server

> Date: 2026-06-28  
> Scope: Adding `services/axum` as the fifth language sidecar in the Hallelujah monorepo

---

## 1. Overview

The Axum sidecar (`sc-axum`) is a standalone Rust HTTP server that follows the same contract as the other sidecars (Gin, Express, FastAPI, Nest). It binds to `127.0.0.1` and exposes:

| Endpoint | Response |
|----------|----------|
| `GET /health` | `{ "ok": true }` |
| `GET /meta` | `{ "service": "axum", "version": "0.1.0" }` |

Port: **7105** (registered in `packages/contracts/ports.yaml`).

---

## 2. Project Layout

```
services/axum/
├── Cargo.toml
├── Makefile
└── src/
    └── main.rs
```

### 2.1 Dependencies

- **axum** — HTTP router and handlers
- **tokio** — async runtime (`#[tokio::main]`, signal handling)
- **tower-http** — CORS middleware layer
- **serde / serde_json** — JSON serialization

### 2.2 Port Configuration

The server reads `SIDECAR_PORT` from the environment (set by Tauri spawn or `dev.sh`). Default fallback: `7105`.

```rust
fn port() -> u16 {
    std::env::var("SIDECAR_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(7105)
}
```

### 2.3 Graceful Shutdown

The server listens for `SIGTERM` (Unix) and `Ctrl+C`, then shuts down via `axum::serve(...).with_graceful_shutdown(...)`.

---

## 3. Integration Checklist

When Axum was added, the following files were updated:

| File | Change |
|------|--------|
| `packages/contracts/ports.yaml` | Registered `axum` on port 7105 |
| `apps/desktop/src-tauri/tauri.conf.json` | Added `binaries/sc-axum` to `externalBin` |
| `apps/desktop/src-tauri/capabilities/default.json` | Added `sc-axum` shell sidecar permission |
| `scripts/build-sidecars.sh` | Added `make build-sidecar` for axum |
| `scripts/dev.sh` | Added `cargo run` startup |
| `Makefile` | Added `axum` target |
| `packages/api-client/src/axum.ts` | HTTP client for the React layer |
| `pnpm generate` | Regenerated Rust registry and TS constants |

---

## 4. Build Commands

```bash
# Dev — run from source
cd services/axum && SIDECAR_PORT=7105 cargo run

# Production sidecar binary → Tauri binaries/
make axum
# or
cd services/axum && make build-sidecar
```

The `build-sidecar` target compiles in release mode and copies the binary to:

```
apps/desktop/src-tauri/binaries/sc-axum-{target-triple}
```

---

## 5. Why Axum as a Sidecar (vs. Embedded in Tauri)

Axum can run inside the Tauri Rust process (same-process localhost server). This project uses the **sidecar pattern** instead so that:

- All language stacks share one architecture (standalone HTTP on `127.0.0.1`)
- Rust HTTP can be studied independently, like Go/Python/Node sidecars
- Build and lifecycle match `externalBin` + shell spawn conventions

For embedded Axum inside `src-tauri/`, see `01_tauri_learning_stack_report.md` Pattern B.
