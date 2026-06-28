# Troubleshooting Guide

> Date: 2026-06-28  
> Scope: Common development issues and how to resolve them

---

## 1. Tauri Build: `resource path binaries/sc-* doesn't exist`

**Symptom:**

```
resource path `binaries/sc-axum-aarch64-apple-darwin` doesn't exist
```

**Cause:** `externalBin` in `tauri.conf.json` requires a file at compile time.

**Fix:**

```bash
./scripts/ensure-sidecar-binaries.sh
# or simply run pnpm dev (calls ensure automatically)
```

Placeholders are created for any missing binary. For production, run `pnpm build:sidecars`.

---

## 2. All Sidecars Offline in UI, but `curl` Works

**Symptom:** Data Flow Monitor shows red/offline; terminal `curl` returns `{"ok":true}`.

**Cause A — Missing CORS:** WebView `fetch` is blocked; `curl` is not.

**Fix:** Ensure CORS middleware is present (see `08_cors_and_webview_fetch.md`). Verify:

```bash
curl -s -D - -H "Origin: http://localhost:5173" \
  http://127.0.0.1:7101/health -o /dev/null | grep -i access-control
```

**Cause B — Stale processes without CORS:**

**Fix:**

```bash
./scripts/free-sidecar-ports.sh
pnpm dev
```

---

## 3. Some Sidecars Online, Others Offline (Mixed State)

**Symptom:** express/fastapi online; gin/axum/nest offline (or vice versa).

**Cause:** Old processes hold ports. New `dev.sh` startups fail to bind; health wait passes against old servers.

**Fix:** Restart with port cleanup (built into `dev.sh` since `free-sidecar-ports.sh` was added):

```bash
# Ctrl+C existing pnpm dev, then:
pnpm dev
```

Look for `[dev] freeing port ...` in the log.

---

## 4. `wait-for-sidecars` Timeout

**Symptom:**

```
[dev] ERROR: timed out waiting for sidecars
  DOWN fastapi (:7103)
```

**Per-service checks:**

| Service | Common issue | Fix |
|---------|--------------|-----|
| gin | Go not installed | `brew install go` |
| express | deps missing | `pnpm install` |
| fastapi | uvicorn not installed | `python3 -m pip install -e services/fastapi` |
| nest | slow first compile | wait longer or increase `SIDECAR_WAIT_TIMEOUT=120` |
| axum | slow `cargo run` | first build takes time; ensure Rust toolchain installed |

Run sidecars individually to see stderr:

```bash
cd services/gin && SIDECAR_PORT=7101 go run ./cmd/server
```

---

## 5. Port Already in Use

**Symptom:** `bind: address already in use` in sidecar logs.

**Check:**

```bash
pnpm check:ports
lsof -i :7101
```

**Fix:**

```bash
./scripts/free-sidecar-ports.sh
```

---

## 6. FastAPI: `uvicorn: command not found`

**Cause:** `uvicorn` executable not on `PATH`.

**Fix:** `dev.sh` uses `python3 -m uvicorn` and auto-installs deps. Manual run:

```bash
python3 -m pip install -e services/fastapi
SIDECAR_PORT=7103 python3 -m uvicorn app.main:app \
  --host 127.0.0.1 --port 7103 --app-dir services/fastapi
```

---

## 7. Tauri Spawns Sidecars During Dev (Port Conflict)

**Symptom:** Duplicate listeners; `go run` fails while a `server` or `sc-gin` process holds 7101.

**Cause:** `TAURI_SKIP_SIDECAR_SPAWN` not set; real binary in `binaries/` gets spawned.

**Fix:** Always use `pnpm dev` (sets the env var). Do not run `pnpm dev:desktop` unless sidecars are already up and spawn is intentionally disabled.

---

## 8. `invoke("get_sidecar_status")` Fails in Browser

**Symptom:** IPC errors in Traffic Log when opening `http://localhost:5173` directly in a browser.

**Cause:** Tauri commands only work inside the Tauri WebView, not a standalone browser tab.

**Fix:** Use `pnpm dev` (launches Tauri window), not Vite alone.

---

## 9. Nest Shows Offline After CORS Change

**Cause:** Nest watch did not restart, or stale `node` process.

**Fix:**

```bash
./scripts/free-sidecar-ports.sh
pnpm dev
```

Confirm CORS header:

```bash
curl -s -D - -H "Origin: http://localhost:5173" \
  http://127.0.0.1:7104/health -o /dev/null | grep -i access-control
```

---

## 10. Quick Health Check (All Ports)

```bash
for p in 7101 7102 7103 7104 7105; do
  printf ":%s → " "$p"
  curl -sf "http://127.0.0.1:$p/health" && echo || echo "DOWN"
done
```

All should print `{"ok":true}` (or equivalent JSON) when `pnpm dev` is healthy.

---

## 11. Regenerate After `ports.yaml` Edit

If ports or binary names change:

```bash
pnpm generate
pnpm check:ports
```

Then update any hand-maintained files (capabilities, `dev.sh` if not using dynamic port parsing) and restart dev.
