# Sidecar Port Registry and Code Generation

> Date: 2026-06-29  
> Scope: `packages/contracts/ports.yaml` as the source of truth for generated sidecar metadata

---

## 1. Purpose

Every sidecar must agree on:

- **Service key** — logical name used in frontend config and `/meta` payloads (`gin`, `express`, ...)
- **Runtime ID** — identifier used by the Rust registry and sidecar spawn logs (`sc-gin`, `sc-express`, ...)
- **Port** — localhost bind port
- **Health path** — endpoint Tauri uses for readiness probes
- **Binary name** — Tauri `externalBin` base name (without target triple suffix)

These values live in a single file so that generated Rust/TypeScript metadata and port-based helper scripts stay in sync. Service startup/build wiring and per-client `BASE` URLs are still maintained manually.

---

## 2. Registry File

`packages/contracts/ports.yaml`:

```yaml
services:
  gin:
    id: sc-gin
    port: 7101
    healthPath: /health
    binaryName: sc-gin

  express:
    id: sc-express
    port: 7102
    healthPath: /health
    binaryName: sc-express

  # ... fastapi (7103), nest (7104), axum (7105)
```

### Naming map

| Concept | Example | Used by |
|---------|---------|---------|
| Service key | `gin` | `SIDECAR_PORTS.gin`, `/meta.service`, UI config |
| Runtime ID | `sc-gin` | Rust `SIDECARS`, spawn logs, placeholder binaries |
| Binary name | `sc-gin` | Tauri `externalBin`, packaged artifact base name |

### Port allocation

| Service | Port | Language |
|---------|------|----------|
| gin | 7101 | Go |
| express | 7102 | Node |
| fastapi | 7103 | Python |
| nest | 7104 | TypeScript (NestJS) |
| axum | 7105 | Rust |

Ports are in the `71xx` range to avoid collisions with common dev servers (3000, 5173, 8080).

---

## 3. Code Generator

Run from the repo root:

```bash
pnpm generate
# equivalent: node tools/sidecar-registry/generate.mjs
```

### Generated outputs

| Output | Purpose |
|--------|---------|
| `packages/api-client/src/constants.ts` | `SIDECAR_PORTS` and `SidecarId` type for the frontend |
| `apps/desktop/src-tauri/src/sidecar/registry.rs` | `SIDECARS` slice for Rust spawn and health checks |

The generator does **not** rewrite the hardcoded `BASE` URLs in `packages/api-client/src/*.ts`; those client modules are still hand-maintained.

### Example generated Rust

```rust
pub const SIDECARS: &[SidecarEntry] = &[
    SidecarEntry { id: "sc-gin", binary: "sc-gin", port: 7101, health_path: "/health" },
    // ...
];
```

### Example generated TypeScript

```typescript
export const SIDECAR_PORTS = {
  gin: 7101,
  express: 7102,
  fastapi: 7103,
  nest: 7104,
  axum: 7105,
} as const;
```

---

## 4. Consumers of `ports.yaml`

| Consumer | Usage |
|----------|-------|
| `scripts/dev.sh` | Parses ports to set `SIDECAR_PORT` per service; the service command list itself is still manual |
| `scripts/wait-for-sidecars.sh` | Health-check loop before Tauri starts; the service list is still manual |
| `scripts/free-sidecar-ports.sh` | Kills listeners on registered ports |
| `scripts/ensure-sidecar-binaries.sh` | Creates placeholder binaries per `binaryName` |
| `scripts/check-ports.mjs` | Pre-flight check for port conflicts |
| `tools/sidecar-registry/generate.mjs` | Emits Rust + TS constants |

---

## 5. Adding a New Sidecar

1. Add an entry to `packages/contracts/ports.yaml` (pick the next free port).
2. Run `pnpm generate`.
3. Wire the new service into `dev.sh`, `build-sidecars.sh`, `tauri.conf.json`, and capabilities.
4. Add or update the HTTP client in `packages/api-client/`, including its manual `BASE` URL.
5. Run `pnpm check:ports` to verify the port is free.
6. Add `packages/contracts/openapi/<service>.yaml` with at least `/health` and `/meta`.
7. Add or update CI coverage so the service is built or syntax-checked on pull requests.

### Registry drift checks

After editing `ports.yaml`, verify these files change together:

| File | Expected update |
|------|-----------------|
| `packages/api-client/src/constants.ts` | Port constants and `SidecarId` union |
| `apps/desktop/src-tauri/src/sidecar/registry.rs` | Rust sidecar registry used by spawn and health checks |
| `packages/api-client/src/<service>.ts` | Manual client `BASE` URL if a port changed, or a new client file if a service was added |
| `apps/desktop/src-tauri/tauri.conf.json` | `externalBin` entry for production packaging |
| `apps/desktop/src-tauri/capabilities/default.json` | Shell permission for the sidecar binary |

If the generated constants changed but `tauri.conf.json` or capabilities did not, production packaging may still miss the new sidecar even though dev mode works.

See `02_monorepo_folder_structure.md` §6 for the full checklist.
