# API Client Package (`@hallelujah/api-client`)

> Date: 2026-06-28  
> Scope: Shared TypeScript HTTP clients for calling sidecars from the React layer

---

## 1. Location

```
packages/api-client/
├── package.json
├── tsconfig.json
└── src/
    ├── index.ts       # Re-exports all clients + constants
    ├── constants.ts   # AUTO-GENERATED SIDECAR_PORTS
    ├── gin.ts
    ├── express.ts
    ├── fastapi.ts
    ├── nest.ts
    └── axum.ts
```

Workspace dependency in `apps/desktop/package.json`:

```json
"@hallelujah/api-client": "workspace:*"
```

---

## 2. Client Pattern

Each sidecar has a thin client module:

```typescript
const BASE = "http://127.0.0.1:7101";

export const ginClient = {
  health: () => get<HealthResponse>("/health"),
  meta: () => get<MetaResponse>("/meta"),
};
```

Shared response types per client:

```typescript
interface HealthResponse { ok: boolean; }
interface MetaResponse { service: string; version: string; }
```

Errors throw with a descriptive message (`sc-gin /health → 404`).

---

## 3. Generated Constants

`pnpm generate` writes `constants.ts`:

```typescript
export const SIDECAR_PORTS = {
  gin: 7101,
  express: 7102,
  fastapi: 7103,
  nest: 7104,
  axum: 7105,
} as const;

export type SidecarId = keyof typeof SIDECAR_PORTS;
```

Exported from `index.ts` for use in the desktop app and future packages.

`pnpm generate` updates `constants.ts`, but it does not rewrite the hardcoded `BASE` values inside `gin.ts`, `express.ts`, `fastapi.ts`, `nest.ts`, or `axum.ts`.

---

## 4. Usage in the Desktop App

```typescript
import { ginClient, SIDECAR_PORTS } from "@hallelujah/api-client";

const health = await ginClient.health();
const meta = await ginClient.meta();
```

Re-exported via `apps/desktop/src/lib/api/index.ts` for convenience.

The Data Flow Monitor (`lib/sidecars.ts`) maps each `SidecarId` to its client for automated probing.

---

## 5. Adding a Client for a New Sidecar

1. Add port entry to `packages/contracts/ports.yaml`.
2. Run `pnpm generate`.
3. Create `packages/api-client/src/{name}.ts` following the existing pattern, including the service's `BASE` URL.
4. Export from `index.ts`.
5. Add to `SIDECAR_CONFIG` in `apps/desktop/src/lib/sidecars.ts`.

---

## 6. Design Notes

- Clients use **native `fetch`** — no axios dependency.
- Base URLs are currently hardcoded per client. `pnpm generate` does not update them yet, so port changes still require manual edits in each client module.
- All requests target `127.0.0.1` only — appropriate for local sidecars, not remote APIs.

---

## 7. Typecheck

```bash
pnpm --filter @hallelujah/api-client typecheck
```

Or from desktop:

```bash
cd apps/desktop && pnpm exec tsc --noEmit
```
