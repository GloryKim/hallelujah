# Tauri + Multi-Sidecar Monorepo Folder Structure Guide

> Date: 2026-06-28  
> Audience: A project where a Tauri desktop app communicates with **continuously growing sidecars** such as Go (Gin), JS (Express), Python (FastAPI), and TS (NestJS + React)

---

## 1. Design Principles

### 1.1 Layer Separation

| Layer | Role | Location |
|--------|------|----------|
| **Desktop Shell** | UI + sidecar lifecycle management | `apps/desktop/` |
| **Sidecar Services** | Language-specific standalone API servers/binaries | `services/<name>/` |
| **Contracts** | API specs, ports, shared types | `packages/contracts/` |
| **Tooling** | Build, bundle, dev orchestration | `tools/`, `scripts/` |

### 1.2 Why a Monorepo

- As sidecars grow, **ports, versions, and build artifacts** must be managed in one place
- Tauri `externalBin` paths and **platform-specific binary naming** rules must be unified
- When the frontend (React) calls multiple sidecar APIs, a **single contract (OpenAPI)** makes maintenance easier

### 1.3 Sidecar Addition Rules (team conventions)

1. New sidecar = **one** folder under `services/<service-id>/`
2. Port registration in `packages/contracts/ports.yaml` is **required**
3. Adding `packages/contracts/openapi/<service-id>.yaml` is **recommended**
4. Put **build artifacts only** in `apps/desktop/src-tauri/binaries/`; keep source in `services/`
5. Default communication: **HTTP on `127.0.0.1`** (local only)

---

## 2. Recommended Folder Tree (Full)

```
hallelujah/
├── apps/
│   └── desktop/                      # Tauri + React (main desktop app)
│       ├── package.json
│       ├── vite.config.ts
│       ├── src/                      # React UI
│       │   ├── main.tsx
│       │   ├── app/
│       │   ├── features/
│       │   └── lib/
│       │       └── api/              # sidecar clients (generated or manual)
│       └── src-tauri/
│           ├── Cargo.toml
│           ├── tauri.conf.json
│           ├── capabilities/
│           ├── src/
│           │   ├── main.rs
│           │   ├── lib.rs
│           │   └── sidecar/          # sidecar spawn, shutdown, health checks
│           └── binaries/             # built sidecar binaries (gitignored)
│               ├── sc-gin-{target-triple}
│               ├── sc-express-{target-triple}
│               ├── sc-fastapi-{target-triple}
│               └── sc-nest-{target-triple}
│
├── services/                         # per-language sidecar source (keep adding)
│   ├── gin/                          # Go + Gin
│   │   ├── go.mod
│   │   ├── cmd/
│   │   │   └── server/
│   │   │       └── main.go
│   │   ├── internal/
│   │   └── Makefile                  # single-binary build
│   │
│   ├── express/                      # Node + Express
│   │   ├── package.json
│   │   ├── src/
│   │   │   ├── index.ts
│   │   │   └── routes/
│   │   └── scripts/
│   │       └── build-sidecar.mjs     # binary packaging via pkg, etc.
│   │
│   ├── fastapi/                      # Python + FastAPI
│   │   ├── pyproject.toml
│   │   ├── app/
│   │   │   └── main.py
│   │   └── scripts/
│   │       └── build-sidecar.sh      # PyInstaller, etc.
│   │
│   └── nest/                         # NestJS (+ optional React admin)
│       ├── package.json
│       ├── apps/
│       │   ├── api/                  # NestJS API (sidecar core)
│       │   └── admin-web/            # Nest-specific React (optional)
│       └── scripts/
│           └── build-sidecar.mjs
│
├── packages/
│   ├── contracts/                    # shared contracts
│   │   ├── ports.yaml                # per-service port, name, health path
│   │   ├── openapi/
│   │   │   ├── gin.yaml
│   │   │   ├── express.yaml
│   │   │   ├── fastapi.yaml
│   │   │   └── nest.yaml
│   │   └── README.md
│   │
│   └── api-client/                   # (optional) OpenAPI → TS client output
│       ├── package.json
│       └── src/
│
├── tools/
│   └── sidecar-registry/             # read ports.yaml and generate types/constants
│       └── generate.mjs
│
├── scripts/
│   ├── dev.sh                        # full dev orchestration
│   ├── build-sidecars.sh             # build all sidecars → copy to binaries/
│   └── check-ports.mjs               # port conflict checks
│
├── docs/
│   ├── monorepo-folder-structure.md  # this document
│   └── tauri-learning-stack-report.md
│
├── .github/
│   └── workflows/
│       ├── desktop.yml
│       └── sidecars.yml
│
├── package.json                      # root workspace (pnpm/bun/npm)
├── pnpm-workspace.yaml               # or bun workspaces
├── Makefile                          # cross-language shortcut commands
└── README.md
```

---

## 3. Folder Roles in Detail

### 3.1 `apps/desktop/` — Tauri + React (center)

**What it does**

- User UI (React + TypeScript)
- Sidecar process **start, stop, restart**
- Sidecar cleanup on app exit (graceful shutdown)
- (Optional) OS-native features via Rust `#[tauri::command]`

**What it does not do**

- Duplicate Gin/Express/FastAPI/Nest business logic in Rust

```
apps/desktop/src-tauri/src/sidecar/
├── mod.rs
├── registry.rs       # sidecar list based on ports.yaml
├── spawn.rs          # Command::sidecar / Rust Command spawn
└── health.rs         # poll GET /health before UI is ready
```

**Tauri config note** (`tauri.conf.json`)

```json
{
  "bundle": {
    "externalBin": [
      "binaries/sc-gin",
      "binaries/sc-express",
      "binaries/sc-fastapi",
      "binaries/sc-nest"
    ]
  }
}
```

Example actual binary filename: `sc-gin-x86_64-apple-darwin` (Tauri appends the target triple automatically)

---

### 3.2 `services/<name>/` — one sidecar = one folder

Whenever you add a new stack, **always follow the same pattern**.

| Service ID | Folder | Stack | Default Port (example) |
|------------|--------|-------|------------------------|
| `gin` | `services/gin/` | Go + Gin | 7101 |
| `express` | `services/express/` | Node + Express | 7102 |
| `fastapi` | `services/fastapi/` | Python + FastAPI | 7103 |
| `nest` | `services/nest/` | NestJS (+ React admin) | 7104 |

#### Shared sidecar contract (required for all services)

```
GET /health          → 200 { "ok": true }
GET /meta            → 200 { "service": "gin", "version": "0.1.0" }
Bind address         → 127.0.0.1 only
Port                 → env var SIDECAR_PORT (default from ports.yaml)
Shutdown             → graceful shutdown on SIGTERM
```

With this contract in place, Tauri `sidecar/health.rs` works **language-agnostically**.

---

### 3.3 Per-language layout under `services/`

#### Go (Gin) — `services/gin/`

```
services/gin/
├── go.mod
├── cmd/server/main.go      # entrypoint (sidecar binary)
├── internal/
│   ├── router/
│   ├── handler/
│   └── config/
└── Makefile
```

- Build: `go build -o sc-gin ./cmd/server`
- Go produces a **single static binary**, which makes it one of the best fits for sidecars

#### Express — `services/express/`

```
services/express/
├── package.json
├── src/
│   ├── index.ts
│   ├── app.ts
│   └── routes/
└── scripts/build-sidecar.mjs
```

- Dev: `tsx src/index.ts` (alongside Tauri dev)
- Release: build `sc-express` binary via `pkg` / `nexe`, etc.
- Source lives in `services/`; artifacts go only to `apps/desktop/src-tauri/binaries/`

#### FastAPI — `services/fastapi/`

```
services/fastapi/
├── pyproject.toml
├── app/
│   ├── main.py
│   └── routers/
└── scripts/
    └── build-sidecar.sh
```

- Dev: `uvicorn app.main:app --host 127.0.0.1 --port $SIDECAR_PORT`
- Release: PyInstaller / cx_Freeze, etc. (Python sidecars tend to be large → consider porting to Go/Rust when possible)

#### NestJS + React — `services/nest/`

```
services/nest/
├── package.json
├── apps/
│   ├── api/                # Nest API → sidecar core
│   │   └── src/main.ts
│   └── admin-web/          # Nest-specific React (admin UI)
│       └── src/
└── scripts/build-sidecar.mjs
```

**Important:** it is easy to end up with React in two places

| React App | Location | Purpose |
|-----------|----------|---------|
| **Main desktop UI** | `apps/desktop/src/` | App shown in the Tauri window |
| **Nest admin UI** | `services/nest/apps/admin-web/` | Admin screen for the Nest API (optional) |

- Keep **one** main desktop UI (`apps/desktop`)
- Treat Nest React as “extra UI served by the Nest service on localhost”, or merge it into the desktop UI later

---

### 3.4 `packages/contracts/` — hub for growing sidecars

As sidecars grow, **this folder becomes the most important**.

#### `ports.yaml` (single source of truth)

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

  fastapi:
    id: sc-fastapi
    port: 7103
    healthPath: /health
    binaryName: sc-fastapi

  nest:
    id: sc-nest
    port: 7104
    healthPath: /health
    binaryName: sc-nest
```

- When adding a sidecar, **add one entry here** → scripts generate TS/Rust constants
- Keep port ranges separate from Vite dev server `5173`, Axum (if used later) `7000`, etc. (`7100` range = sidecars)

#### `openapi/<service>.yaml`

- Auto-generate frontend `packages/api-client`
- Sidecar teams update the contract first when APIs change

---

### 3.5 `packages/api-client/` (optional, strongly recommended)

```
packages/api-client/
├── package.json
└── src/
    ├── gin.ts
    ├── express.ts
    └── index.ts
```

In `apps/desktop`:

```typescript
import { ginClient } from "@hallelujah/api-client";

const res = await ginClient.getUser(1);
```

Even with 10 sidecars, UI code only needs to import **client modules**.

---

## 4. Communication Flow

```
┌─────────────────────────────────────────────────────────┐
│  apps/desktop (Tauri + React)                           │
│  ┌─────────────┐    invoke     ┌──────────────────────┐ │
│  │ React UI    │ ────────────► │ Rust (commands,      │ │
│  │             │               │  sidecar lifecycle)  │ │
│  └──────┬──────┘               └──────────┬───────────┘ │
│         │ fetch 127.0.0.1:710x             │ spawn       │
└─────────┼──────────────────────────────────┼────────────┘
          │                                  │
          ▼                                  ▼
   ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐
   │ gin:7101   │  │ express    │  │ fastapi    │  │ nest:7104  │
   │            │  │ :7102      │  │ :7103      │  │            │
   └────────────┘  └────────────┘  └────────────┘  └────────────┘
        services/gin   services/express  services/fastapi  services/nest
```

- **UI → Sidecar:** `fetch("http://127.0.0.1:7101/...")` (or api-client)
- **UI → Rust:** `invoke("open_file_dialog")` (OS features)
- **Rust → Sidecar:** responsible for spawn only; React can call business HTTP directly

---

## 5. Dev vs Prod Modes

### 5.1 Dev mode — run sidecars directly from source

Example `scripts/dev.sh` flow:

1. Load `ports.yaml`
2. Start each `services/*` in **source mode** in the background  
   - Go: `go run ./cmd/server`  
   - Express: `pnpm --filter express dev`  
   - FastAPI: `uvicorn ...`  
   - Nest: `pnpm --filter nest-api dev`
3. Run `pnpm tauri dev` in `apps/desktop`

At this stage you can develop without `binaries/` (shorter build cycle).

### 5.2 Prod mode — binary sidecars

1. `scripts/build-sidecars.sh` → build each service
2. Copy artifacts to `apps/desktop/src-tauri/binaries/sc-<name>-<triple>`
3. `pnpm tauri build`

Tauri packages them together via `externalBin`.

---

## 6. New Sidecar Checklist

When adding a new stack (e.g. Rust Actix, Elixir):

- [ ] Create `services/<new-id>/` (copy the pattern above)
- [ ] Register port and `binaryName` in `packages/contracts/ports.yaml`
- [ ] Add `packages/contracts/openapi/<new-id>.yaml`
- [ ] Implement `GET /health` and `127.0.0.1` binding
- [ ] Add to `apps/desktop/src-tauri/tauri.conf.json` → `externalBin`
- [ ] Add shell spawn permission in `apps/desktop/src-tauri/capabilities/`
- [ ] Add entry to `apps/desktop/src-tauri/src/sidecar/registry.rs` (or auto-generate from yaml)
- [ ] Add target to `scripts/dev.sh` and `scripts/build-sidecars.sh`
- [ ] Add client to `packages/api-client`
- [ ] Add build job to CI workflow

---

## 7. Git / Build Artifact Policy

Recommended `.gitignore` entries:

```
apps/desktop/src-tauri/binaries/*
!apps/desktop/src-tauri/binaries/.gitkeep

# per language
services/*/dist/
services/*/build/
services/*/__pycache__/
services/*/node_modules/
```

- Commit **source** to git
- Build **sidecar binaries** in CI or manage them as release artifacts

---

## 8. Root Workspace Setup Example

### `pnpm-workspace.yaml`

```yaml
packages:
  - "apps/*"
  - "packages/*"
  - "services/express"
  - "services/nest"
  - "services/nest/apps/*"
```

Go / Python sit outside the pnpm workspace but are integrated via `Makefile` / `scripts/dev.sh`.

### Root `package.json` scripts (example)

```json
{
  "scripts": {
    "dev": "./scripts/dev.sh",
    "dev:desktop": "pnpm --filter desktop tauri dev",
    "build:sidecars": "./scripts/build-sidecars.sh",
    "build": "pnpm build:sidecars && pnpm --filter desktop tauri build",
    "check:ports": "node scripts/check-ports.mjs"
  }
}
```

---

## 9. Anti-Patterns to Avoid

| Anti-pattern | Why |
|--------------|-----|
| Mixing sidecar source into `src-tauri/` | Language, build, and dependency hell |
| Hardcoding ports in each service | Collisions and environment-specific breakage |
| Different health paths per sidecar | Tauri startup wait logic becomes complex |
| Merging Nest React and desktop React into one folder | Build, routing, and deployment boundaries collapse |
| Adding a sidecar per feature | Process count, memory, and startup time explode |
| Duplicating the same API across Axum + Express + Gin | Unmaintainable |

**Recommendation:** split sidecars by **language/domain**, and do not duplicate the same domain API across multiple sidecars.

---

## 10. Phased Rollout (for this repo)

### Phase 0 — skeleton only

```
apps/desktop/          # Tauri + React
packages/contracts/    # ports.yaml
scripts/dev.sh
```

### Phase 1 — one sidecar (Go Gin recommended)

- Easiest way to validate the binary build pipeline
- `services/gin/` + spawn + health check

### Phase 2 — Node family (Express, Nest)

- Establish pkg/bundle scripts
- `services/express/`, `services/nest/`

### Phase 3 — Python (FastAPI)

- Package sidecar with PyInstaller, etc.
- Measure size and startup time before deciding to keep it

### Phase 4 — automation

- OpenAPI → api-client generation
- `ports.yaml` → Rust/TS code generation (`tools/sidecar-registry/`)

---

## 11. Final Summary

| Question | Answer |
|----------|--------|
| Where is the Tauri app? | `apps/desktop/` |
| Where do sidecars keep getting added? | `services/<service-id>/` |
| Where are port/name rules? | `packages/contracts/ports.yaml` |
| Where are API contracts? | `packages/contracts/openapi/` |
| Where do built binaries go? | `apps/desktop/src-tauri/binaries/` (artifacts only) |
| Where is the main React UI? | `apps/desktop/src/` — **one** |
| Where is Nest-specific React? | `services/nest/apps/admin-web/` (optional, separate) |

**In one line:** `apps/desktop` = shell, UI, sidecar management; `services/*` = growing language-specific backends; `packages/contracts` = shared connection rules everyone follows.

---

## 12. Next Steps (optional)

Scaffolding order for this structure:

1. Initialize `apps/desktop` with Tauri + React
2. Create `packages/contracts/ports.yaml`
3. Add minimal `/health` API in `services/gin`
4. Wire up `scripts/dev.sh` + sidecar spawn

If needed, empty folders and config files can be generated in the repo following the order above.
