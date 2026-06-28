# Development Workflow

> Date: 2026-06-28  
> Scope: `pnpm dev` orchestration and supporting scripts

---

## 1. Quick Start

```bash
pnpm install
pnpm dev
```

This runs `scripts/dev.sh`, which:

1. Ensures placeholder sidecar binaries exist (for Tauri compile).
2. Frees any stale processes on sidecar ports.
3. Starts all five sidecars from source in the background.
4. Waits until every sidecar responds on `GET /health`.
5. Launches `pnpm tauri dev` with `TAURI_SKIP_SIDECAR_SPAWN=1`.

Press **Ctrl+C** once to stop Tauri and all sidecars.

---

## 2. Script Reference

| Script | Role |
|--------|------|
| `scripts/dev.sh` | Main dev orchestrator |
| `scripts/ensure-sidecar-binaries.sh` | Create empty `binaries/sc-*-{triple}` if missing |
| `scripts/free-sidecar-ports.sh` | Kill listeners on ports from `ports.yaml` |
| `scripts/wait-for-sidecars.sh` | Poll `/health` until all ready (60s timeout) |
| `scripts/build-sidecars.sh` | Build all production sidecar binaries |
| `scripts/check-ports.mjs` | Exit 1 if any registered port is already in use |

---

## 3. `dev.sh` Startup Detail

### Per-service commands

| Service | Command |
|---------|---------|
| gin | `SIDECAR_PORT=7101 go run ./cmd/server` |
| express | `SIDECAR_PORT=7102 pnpm --filter express dev` |
| fastapi | `python3 -m uvicorn app.main:app --host 127.0.0.1 --port 7103` |
| nest | `SIDECAR_PORT=7104 pnpm --filter nest-api dev` |
| axum | `SIDECAR_PORT=7105 cargo run` |

### FastAPI dependency bootstrap

If `uvicorn` is not importable, dev.sh runs:

```bash
python3 -m pip install -q -e services/fastapi
```

### Health wait

`wait-for-sidecars.sh` loops every 500ms until all five services return HTTP 200 on their health path. On timeout it prints which services are still down and exits with code 1.

---

## 4. Makefile Targets

```bash
make dev              # ./scripts/dev.sh
make build            # build sidecars + tauri build
make build-sidecars   # all sidecar binaries only
make generate         # regenerate registry from ports.yaml
make check-ports      # port conflict check
make clean            # remove binaries + cargo/go clean

make gin | express | fastapi | nest | axum   # individual sidecar build
```

---

## 5. Root `package.json` Scripts

| Script | Command |
|--------|---------|
| `pnpm dev` | Full stack (sidecars + Tauri) |
| `pnpm dev:desktop` | Tauri only (sidecars must be running separately) |
| `pnpm build` | Production sidecars + desktop bundle |
| `pnpm build:sidecars` | Sidecar binaries only |
| `pnpm generate` | Regenerate port registry artifacts |
| `pnpm check:ports` | Port availability check |

---

## 6. Dev vs. Production

| Aspect | Dev (`pnpm dev`) | Production (`pnpm build`) |
|--------|------------------|---------------------------|
| Sidecar source | Run from source (`go run`, etc.) | Compiled binaries in `binaries/` |
| Tauri spawn | Skipped (`TAURI_SKIP_SIDECAR_SPAWN`) | Enabled — spawns `externalBin` |
| Binary files | Placeholders (0 bytes) OK | Real executables required |
| Frontend | Vite HMR at `localhost:5173` | Static bundle in `apps/desktop/dist` |

---

## 7. Expected Log Sequence

```
[ensure-sidecar-binaries] created placeholder ...   (only if missing)
[dev] freeing port 7101 (pid ...)                   (only if stale)
[dev] starting sidecars...
[dev] waiting for sidecars (timeout 60s)...
[dev] all sidecars ready
[dev] starting Tauri...
[sidecar] spawn skipped (TAURI_SKIP_SIDECAR_SPAWN)
```

If `all sidecars ready` never appears, see `11_troubleshooting.md`.
