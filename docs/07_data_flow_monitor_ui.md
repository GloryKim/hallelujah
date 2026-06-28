# Data Flow Monitor UI

> Date: 2026-06-28  
> Scope: React dashboard that visualizes communication between the WebView, Tauri, and sidecars

---

## 1. Purpose

The desktop app includes a **Data Flow Monitor** that makes runtime communication visible:

- Direct HTTP calls from React to each sidecar
- Tauri IPC (`invoke`) for aggregated health status
- Request/response payloads, latency, and success/failure state

Location: `apps/desktop/src/`

---

## 2. File Structure

```
apps/desktop/src/
├── app/
│   ├── App.tsx          # Main dashboard layout
│   └── App.css          # Dark-theme styles
├── components/
│   ├── ArchitectureDiagram.tsx   # Layer diagram with pulse animation
│   ├── SidecarGrid.tsx           # Per-sidecar payload cards
│   └── TrafficLog.tsx            # Scrollable event log
├── hooks/
│   └── useDataFlow.ts            # Polling, logging, state
└── lib/
    ├── sidecars.ts      # SIDECAR_CONFIG (clients + metadata)
    └── traffic.ts       # TrafficEvent types and helpers
```

---

## 3. UI Sections

### 3.1 Header

- Online count (`3/5 sidecars online`)
- Last Tauri probe timestamp
- **Probe now** button — manual refresh

### 3.2 Architecture Diagram

Three-layer visualization:

1. **React UI** — WebView (`fetch` + `invoke`)
2. **Tauri Shell** — Rust (spawn, `reqwest` probes)
3. **Sidecars** — gin, express, fastapi, nest, axum

Active communication paths pulse briefly (HTTP = blue, IPC = purple).

### 3.3 Sidecar Grid

Per-sidecar card showing:

- Status badge (Online / Offline / Unknown)
- Last `/health` JSON payload
- Last `/meta` JSON payload
- Tauri probe result (`{ id, port, ok }`)
- HTTP latency and last check time

### 3.4 Traffic Log

Newest-first event stream. Each entry includes:

| Field | Description |
|-------|-------------|
| `channel` | `http` or `ipc` |
| `direction` | `request`, `response`, or `error` |
| `from` / `to` | Source and destination labels |
| `label` | Human-readable description (method, URL, command) |
| `payload` | JSON body or error message |
| `durationMs` | Round-trip time when available |

---

## 4. Data Flow Hook (`useDataFlow`)

### HTTP probe (per sidecar)

For each entry in `SIDECAR_CONFIG`:

1. Log outbound `GET /health` request.
2. Call `client.health()` via `@hallelujah/api-client`.
3. Log response with payload and latency.
4. Repeat for `GET /meta`.
5. Update sidecar snapshot status (`ok` / `error`).

### IPC probe (Tauri)

1. Log `invoke("get_sidecar_status")` request.
2. Call Tauri command.
3. Log response array.
4. For each row in the response, log synthetic HTTP events representing Rust's internal `reqwest` probes.

### Auto-poll

- Interval: **5 seconds**
- Initial probe on mount
- `probing` ref prevents overlapping manual + interval runs

---

## 5. Dependencies

- `@hallelujah/api-client` — typed HTTP clients per sidecar
- `@tauri-apps/api/core` — `invoke` for IPC

---

## 6. Extending the UI

When adding a new sidecar:

1. Register in `ports.yaml` and run `pnpm generate`.
2. Add client export in `packages/api-client`.
3. Add entry to `lib/sidecars.ts` → `SIDECAR_CONFIG`.
4. The diagram and grid render dynamically from `SIDECAR_CONFIG` — no hardcoded service list in components.
