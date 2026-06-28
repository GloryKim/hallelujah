# CORS and WebView Fetch

> Date: 2026-06-28  
> Scope: Why sidecars appeared offline in the UI while `curl` worked, and how CORS was fixed

---

## 1. Problem

After adding the Data Flow Monitor, sidecars showed **Offline** in the React UI even though:

```bash
curl http://127.0.0.1:7101/health
# → {"ok":true}
```

worked fine from the terminal.

---

## 2. Root Cause: Cross-Origin Requests

In dev mode:

| Layer | Origin |
|-------|--------|
| React (Vite) | `http://localhost:5173` or `http://127.0.0.1:5173` |
| Sidecars | `http://127.0.0.1:7101` … `7105` |

The WebView treats these as **different origins** (scheme/host/port mismatch). Browser security requires sidecars to respond with CORS headers:

```
Access-Control-Allow-Origin: *
```

Without them, `fetch()` fails silently from the UI's perspective (caught as errors in `useDataFlow`), while `curl` — which does not enforce CORS — succeeds.

`invoke()` (Tauri IPC) is not subject to CORS; only direct HTTP from the WebView is.

---

## 3. Fix: CORS on Every Sidecar

### Go (Gin)

`services/gin/internal/middleware/cors.go` — manual middleware:

```go
c.Header("Access-Control-Allow-Origin", "*")
c.Header("Access-Control-Allow-Methods", "GET, OPTIONS")
```

Registered in `router.go` via `r.Use(middleware.CORS())`.

### Express

Inline middleware in `services/express/src/app.ts` before routes.

### FastAPI

```python
from fastapi.middleware.cors import CORSMiddleware
app.add_middleware(CORSMiddleware, allow_origins=["*"], ...)
```

### NestJS

```typescript
app.enableCors({ origin: true, methods: ["GET", "HEAD", "OPTIONS"] });
```

`origin: true` reflects the request origin (e.g. `http://localhost:5173`).

### Axum

```rust
use tower_http::cors::{Any, CorsLayer};
// ...
.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
```

---

## 4. Secondary Issue: Stale Processes

Even after adding CORS to source code, gin/axum/nest could still appear offline because **old processes without CORS** held the ports:

1. `dev.sh` tried `go run` / `cargo run` → bind failed (port in use).
2. `wait-for-sidecars.sh` succeeded because the **old** process still answered `/health`.
3. express/fastapi (freshly started) had CORS → **Online**.
4. gin/axum/nest (stale) had no CORS → **Offline** in UI.

### Fix

`scripts/free-sidecar-ports.sh` runs at the start of `dev.sh` and kills any listener on registered ports before starting new processes.

---

## 5. Verification

After a clean `pnpm dev`, confirm CORS headers:

```bash
curl -s -D - -H "Origin: http://localhost:5173" \
  http://127.0.0.1:7101/health -o /dev/null | grep -i access-control
```

Expected:

```
Access-Control-Allow-Origin: *
```

Repeat for ports 7102–7105.

---

## 6. Production Note

When the frontend is bundled (not served from Vite), the origin changes (e.g. `tauri://localhost`). Sidecars that use `allow_origins: ["*"]` or `origin: true` continue to work. Tighten origins in production if sidecars ever bind beyond `127.0.0.1`.
