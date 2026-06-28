# Sidecar API Contract

> Date: 2026-06-29  
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

This `service` field is the **service key**. It intentionally differs from the Rust runtime ID / binary name, which use values like `sc-gin`.

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

Sidecars should handle `SIGTERM` and `SIGINT` when those signals are delivered:

- Go: `signal.Notify` + `http.Server.Shutdown`
- Node: `process.on("SIGTERM", ...)` + `server.close()`
- Python: uvicorn lifecycle / signal handlers
- Nest: `app.close()` on SIGTERM
- Rust: `axum::serve(...).with_graceful_shutdown(...)`

Current caveat: the desktop shell does not yet retain child handles and explicitly send `SIGTERM` on app exit. In production today, process cleanup primarily relies on OS reclamation.

---

## 6. OpenAPI Contract Files

OpenAPI specs live under:

```
packages/contracts/openapi/
├── gin.yaml
├── express.yaml
├── fastapi.yaml
├── nest.yaml
└── axum.yaml
```

Each spec should describe the same required baseline endpoints (`GET /health`, `GET /meta`) before service-specific endpoints are added. When a sidecar API changes, update the OpenAPI spec in the same change as the service implementation and the matching API client.

The current TypeScript clients in `packages/api-client/` are hand-written, but the OpenAPI files are still the review contract. They make API drift visible and leave room for generated clients later.

### Contract update checklist

1. Update `packages/contracts/ports.yaml` if the service name, port, health path, or binary name changes.
2. Update `packages/contracts/openapi/<service>.yaml` when endpoint shapes change.
3. Run `pnpm generate` after port registry changes.
4. Update `packages/api-client/src/<service>.ts` if the React app consumes the changed endpoint, and update its hardcoded `BASE` URL manually if the port changed.
5. Verify the sidecar still returns `200 OK` from `/health` and a stable `{ service, version }` payload from `/meta`.

---

## 7. Compliance Matrix

| Sidecar | /health | /meta | CORS | SIDECAR_PORT | Signal handlers implemented |
|---------|---------|-------|------|--------------|-----------------------------|
| gin | ✅ | ✅ | ✅ | ✅ | ✅ |
| express | ✅ | ✅ | ✅ | ✅ | ✅ |
| fastapi | ✅ | ✅ | ✅ | ✅ | ✅ |
| nest | ✅ | ✅ | ✅ | ✅ | ✅ |
| axum | ✅ | ✅ | ✅ | ✅ | ✅ |
