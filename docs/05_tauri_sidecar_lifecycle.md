# Tauri Sidecar Lifecycle

> Date: 2026-06-28  
> Scope: How the desktop shell spawns, probes, and communicates with sidecars

---

## 1. Architecture

```
┌─────────────────────────────────────────────────────────┐
│  apps/desktop (Tauri + React WebView)                   │
│                                                         │
│  React UI ──fetch──► http://127.0.0.1:710x             │
│  React UI ──invoke──► get_sidecar_status (Rust IPC)     │
│                                                         │
│  Rust (lib.rs)                                          │
│    ├── spawn_all()     → shell sidecar binaries         │
│    └── check_all()     → reqwest GET /health            │
└─────────────────────────────────────────────────────────┘
         │ HTTP (localhost)
         ▼
┌─────────────────────────────────────────────────────────┐
│  Sidecars: gin · express · fastapi · nest · axum        │
└─────────────────────────────────────────────────────────┘
```

---

## 2. Module Layout

```
apps/desktop/src-tauri/src/sidecar/
├── registry.rs    # AUTO-GENERATED from ports.yaml
├── spawn.rs       # shell.sidecar() + SIDECAR_PORT env
└── health.rs      # reqwest probes for /health
```

---

## 3. Spawn (Production)

`spawn.rs` iterates `SIDECARS` and starts each binary via `tauri_plugin_shell`:

```rust
app.shell()
    .sidecar(entry.binary)?
    .env("SIDECAR_PORT", entry.port.to_string())
    .spawn()?;
```

Requirements:

- Binary must exist at `binaries/sc-{name}-{target-triple}` (see `externalBin` in `tauri.conf.json`).
- Capability `shell:allow-execute` must list each sidecar with `"sidecar": true`.

Each sidecar reads `SIDECAR_PORT` and binds to `127.0.0.1:<port>`.

---

## 4. Dev Mode: Skip Spawn

In development, `scripts/dev.sh` starts sidecars **from source** (`go run`, `cargo run`, etc.). Spawning stale or placeholder binaries would cause port conflicts.

`dev.sh` sets:

```bash
export TAURI_SKIP_SIDECAR_SPAWN=1
```

`lib.rs` checks this env var and skips `spawn_all()` when set:

```rust
let skip_spawn = std::env::var("TAURI_SKIP_SIDECAR_SPAWN").is_ok();
if !skip_spawn {
    sidecar::spawn::spawn_all(&handle, &SIDECARS).await;
}
```

Production builds (`pnpm tauri build`) do not set this variable — binaries are spawned normally.

---

## 5. Health Checks (Rust)

`health.rs` exposes `check_all()`:

1. Builds a `reqwest` client with a 2-second timeout.
2. For each `SidecarEntry`, sends `GET http://127.0.0.1:{port}{health_path}`.
3. Returns `{ id, port, ok }` per sidecar.

### Tauri command

```rust
#[tauri::command]
async fn get_sidecar_status(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let statuses = sidecar::health::check_all(&app).await;
    Ok(serde_json::to_value(statuses)?)
}
```

The React UI calls this via:

```typescript
import { invoke } from "@tauri-apps/api/core";
const rows = await invoke("get_sidecar_status");
```

---

## 6. Placeholder Binaries (Dev Compile)

Tauri requires `externalBin` files to exist at **compile time**, even in dev. Empty placeholder executables are created by `scripts/ensure-sidecar-binaries.sh`:

```
apps/desktop/src-tauri/binaries/sc-gin-aarch64-apple-darwin   (0 bytes, executable)
```

Real servers run from source; placeholders only satisfy the Tauri build check.

---

## 7. Shutdown

Many sidecars implement `SIGTERM` / `SIGINT` handlers themselves, but the Tauri shell currently does **not** retain child handles and does not explicitly send those signals on app exit.

Today, child sidecar processes are reclaimed by the OS. A production app may store child handles and send `SIGTERM` explicitly — currently documented as a future improvement in `spawn.rs`.
