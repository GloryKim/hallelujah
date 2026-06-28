# Documentation Index

> Date: 2026-06-28  
> Hallelujah monorepo — Tauri desktop app with multi-language sidecars

---

## Reading Order

| # | Document | Summary |
|---|----------|---------|
| 01 | [tauri_learning_stack_report.md](./01_tauri_learning_stack_report.md) | Research: React, Express, Tokio, Axum inside Tauri; risks of accumulating stacks |
| 02 | [monorepo_folder_structure.md](./02_monorepo_folder_structure.md) | Folder layout, layer separation, new sidecar checklist |
| 03 | [axum_sidecar.md](./03_axum_sidecar.md) | Rust Axum sidecar (`sc-axum`, port 7105) |
| 04 | [sidecar_port_registry.md](./04_sidecar_port_registry.md) | `ports.yaml` and `pnpm generate` code generation |
| 05 | [tauri_sidecar_lifecycle.md](./05_tauri_sidecar_lifecycle.md) | Spawn, health probes, IPC, dev skip spawn |
| 06 | [dev_workflow.md](./06_dev_workflow.md) | `pnpm dev`, scripts, Makefile, dev vs production |
| 07 | [data_flow_monitor_ui.md](./07_data_flow_monitor_ui.md) | React dashboard for HTTP/IPC visualization |
| 08 | [cors_and_webview_fetch.md](./08_cors_and_webview_fetch.md) | CORS fix, stale process port conflicts |
| 09 | [sidecar_binary_build.md](./09_sidecar_binary_build.md) | Per-language packaging into `binaries/` |
| 10 | [api_client_package.md](./10_api_client_package.md) | `@hallelujah/api-client` HTTP clients |
| 11 | [sidecar_api_contract.md](./11_sidecar_api_contract.md) | `/health`, `/meta`, CORS, shutdown contract |
| 12 | [troubleshooting.md](./12_troubleshooting.md) | Common errors and fixes |

---

## Quick Commands

```bash
pnpm dev              # Start all sidecars + Tauri
pnpm generate         # Regenerate registry from ports.yaml
pnpm build:sidecars   # Build production sidecar binaries
pnpm build            # Full production desktop build
pnpm check:ports      # Verify ports 7101–7105 are free
```

---

## Maintenance Checklist

When changing sidecar behavior, keep these files in sync:

| Change | Files to review |
|--------|-----------------|
| Port, binary name, or health path | `packages/contracts/ports.yaml`, generated Rust/TS registry files, Tauri `externalBin`, shell capabilities |
| HTTP endpoint shape | `packages/contracts/openapi/<service>.yaml`, `packages/api-client/src/<service>.ts`, UI consumers |
| New sidecar | `services/<name>/`, `ports.yaml`, OpenAPI spec, API client, dev/build scripts, Tauri config, CI workflow |
| Production packaging | `scripts/build-sidecars.sh`, `apps/desktop/src-tauri/binaries/`, `.gitignore`, GitHub Actions |

Run `pnpm generate` after registry edits and include the generated files in the same commit.

---

## Current Sidecars

| Service | Port | Stack | Binary |
|---------|------|-------|--------|
| gin | 7101 | Go + Gin | `sc-gin` |
| express | 7102 | Node + Express | `sc-express` |
| fastapi | 7103 | Python + FastAPI | `sc-fastapi` |
| nest | 7104 | NestJS | `sc-nest` |
| axum | 7105 | Rust + Axum | `sc-axum` |

---

## Key Paths

```
apps/desktop/              Tauri + React app
apps/desktop/src-tauri/      Rust backend, binaries/, sidecar/
services/{gin,express,...}/  Sidecar source code
packages/contracts/        ports.yaml (registry)
packages/api-client/         TypeScript HTTP clients
scripts/                     dev.sh, build, port utilities
tools/sidecar-registry/      generate.mjs
```
