# Sidecar API Contract

> Date: 2026-06-28  
> Scope: Shared HTTP contract every sidecar must implement

---

## 1. Binding Rules

| Rule | Value |
|------|-------|
| Host | `127.0.0.1` only (never `0.0.0.0` in this project) |
| Port | From `SIDECAR_PORT` env var; fallback per service default |
| Protocol | HTTP (HTTPS not required for localhost sidecars) |

---

## 2. Required Endpoints

### `GET /health`

**Purpose:** Readiness probe for Tauri (`health.rs`), `wait-for-sidecars.sh`, and the UI.

**Response:** `200 OK`

```json
{ "ok": true }
```

### `GET /meta`

**Purpose:** Service identification for the Data Flow Monitor and debugging.

**Response:** `200 OK`

```json
{
  "service": "<stack-name>",
  "version": "0.1.0"
}
```

Example values for `service`: `"gin"`, `"express"`, `"fastapi"`, `"nest"`, `"axum"`.

---

## 3. CORS Requirements

Because the React WebView calls these endpoints via `fetch`, every sidecar must include CORS headers allowing the dev server origin. Minimum:

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, OPTIONS
```

Handle `OPTIONS` preflight where the framework requires it.

See `08_cors_and_webview_fetch.md` for per-framework implementation.

---

## 4. Environment Variables

| Variable | Set by | Meaning |
|----------|--------|---------|
| `SIDECAR_PORT` | Tauri spawn or `dev.sh` | TCP port to bind |

Sidecars must not hardcode ports in production code except as dev fallbacks.

---

## 5. Graceful Shutdown

Sidecars should handle `SIGTERM` and `SIGINT`:

- Go: `signal.Notify` + `http.Server.Shutdown`
- Node: `process.on("SIGTERM", ...)` + `server.close()`
- Python: uvicorn lifecycle / signal handlers
- Nest: `app.close()` on SIGTERM
- Rust: `axum::serve(...).with_graceful_shutdown(...)`

---

## 6. Future: OpenAPI

`02_monorepo_folder_structure.md` recommends adding specs under:

```
packages/contracts/openapi/
├── gin.yaml
├── express.yaml
└── ...
```

Not yet implemented; clients are currently hand-written in `packages/api-client/`.

---

## 7. Compliance Matrix

| Sidecar | /health | /meta | CORS | SIDECAR_PORT | Graceful shutdown |
|---------|---------|-------|------|--------------|-----------------|
| gin | ✅ | ✅ | ✅ | ✅ | ✅ |
| express | ✅ | ✅ | ✅ | ✅ | ✅ |
| fastapi | ✅ | ✅ | ✅ | ✅ | ✅ |
| nest | ✅ | ✅ | ✅ | ✅ | ✅ |
| axum | ✅ | ✅ | ✅ | ✅ | ✅ |
