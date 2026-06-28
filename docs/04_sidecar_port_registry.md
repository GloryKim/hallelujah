# Sidecar Port Registry and Code Generation

> Date: 2026-06-28  
> Scope: `packages/contracts/ports.yaml` as the single source of truth for sidecar metadata

---

## 1. Purpose

Every sidecar must agree on:

- **ID** — logical name used in logs and health results
- **Port** — localhost bind port
- **Health path** — endpoint Tauri uses for readiness probes
- **Binary name** — Tauri `externalBin` base name (without target triple suffix)

These values live in one file so Rust, TypeScript, and shell scripts stay in sync.

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
| `scripts/dev.sh` | Parses ports to set `SIDECAR_PORT` per service |
| `scripts/wait-for-sidecars.sh` | Health-check loop before Tauri starts |
| `scripts/free-sidecar-ports.sh` | Kills listeners on registered ports |
| `scripts/ensure-sidecar-binaries.sh` | Creates placeholder binaries per `binaryName` |
| `scripts/check-ports.mjs` | Pre-flight check for port conflicts |
| `tools/sidecar-registry/generate.mjs` | Emits Rust + TS constants |

---

## 5. Adding a New Sidecar

1. Add an entry to `packages/contracts/ports.yaml` (pick the next free port).
2. Run `pnpm generate`.
3. Wire the new service into `dev.sh`, `build-sidecars.sh`, `tauri.conf.json`, and capabilities.
4. Add an HTTP client in `packages/api-client/`.
5. Run `pnpm check:ports` to verify the port is free.
6. Add `packages/contracts/openapi/<service>.yaml` with at least `/health` and `/meta`.
7. Add or update CI coverage so the service is built or syntax-checked on pull requests.

### Registry drift checks

After editing `ports.yaml`, verify these files change together:

| File | Expected update |
|------|-----------------|
| `packages/api-client/src/constants.ts` | Port constants and `SidecarId` union |
| `apps/desktop/src-tauri/src/sidecar/registry.rs` | Rust sidecar registry used by spawn and health checks |
| `apps/desktop/src-tauri/tauri.conf.json` | `externalBin` entry for production packaging |
| `apps/desktop/src-tauri/capabilities/default.json` | Shell permission for the sidecar binary |

If the generated constants changed but `tauri.conf.json` or capabilities did not, production packaging may still miss the new sidecar even though dev mode works.

See `02_monorepo_folder_structure.md` §6 for the full checklist.
