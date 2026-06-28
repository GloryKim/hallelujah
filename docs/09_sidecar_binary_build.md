# Sidecar Binary Build and Packaging

> Date: 2026-06-29  
> Scope: How each language sidecar is compiled into a Tauri `externalBin` artifact

---

## 1. Output Convention

All production sidecar binaries land in:

```
apps/desktop/src-tauri/binaries/sc-{name}-{target-triple}
```

Example (Apple Silicon macOS):

```
sc-gin-aarch64-apple-darwin
sc-express-aarch64-apple-darwin
sc-fastapi-aarch64-apple-darwin
sc-nest-aarch64-apple-darwin
sc-axum-aarch64-apple-darwin
```

Tauri resolves the target triple at build time via `rustc -vV | grep host`.

`tauri.conf.json` lists base names only (no triple suffix):

```json
"externalBin": [
  "binaries/sc-gin",
  "binaries/sc-express",
  ...
]
```

---

## 2. Build All Sidecars

```bash
pnpm build:sidecars
# or: ./scripts/build-sidecars.sh
```

Run this before `pnpm tauri build` or in CI (see `.github/workflows/desktop.yml`).

---

## 3. Per-Language Build

### Go (Gin)

```bash
make -C services/gin build-sidecar
```

- `go build -o sc-gin ./cmd/server`
- Copy to `binaries/sc-gin-{triple}`

### Node (Express)

```bash
pnpm --filter express build:sidecar
```

Pipeline (`services/express/scripts/build-sidecar.mjs`):

1. `pnpm build` (TypeScript → `dist/`)
2. `pkg dist/index.js` → standalone executable
3. Copy to `binaries/sc-express-{triple}`

### Python (FastAPI)

```bash
cd services/fastapi && bash scripts/build-sidecar.sh
```

- **PyInstaller** `--onefile` on `app/main.py`
- Output: `dist-bin/sc-fastapi` → copied with triple suffix

### NestJS

```bash
pnpm --filter nest-api build:sidecar
```

- `nest build` → `pkg` or similar bundling (see `services/nest/scripts/build-sidecar.mjs`)
- Copy to `binaries/sc-nest-{triple}`

### Rust (Axum)

```bash
make -C services/axum build-sidecar
```

- `cargo build --release`
- Copy `target/release/sc-axum` → `binaries/sc-axum-{triple}`

---

## 4. Dev Placeholders vs. Real Binaries

| Mode | Binary content | Who serves HTTP |
|------|----------------|-----------------|
| Dev | 0-byte placeholder (or stale partial build) | `dev.sh` source processes |
| Production | Full compiled executable | Tauri `spawn_all()` |

`ensure-sidecar-binaries.sh` creates placeholders **only if the file is missing**. It does not overwrite real builds.

---

## 5. Git Policy

`apps/desktop/src-tauri/binaries/*` is gitignored (except `.gitkeep`). Binaries are build artifacts — reproduce via `pnpm build:sidecars` or CI.

---

## 6. CI Integration

`.github/workflows/desktop.yml`:

1. Install Rust, Node (pnpm), system deps (Linux WebKit).
2. `pnpm build:sidecars`
3. `pnpm --filter desktop tauri build`
4. Upload bundle artifacts per OS matrix (macOS, Ubuntu, Windows).

---

## 7. Clean

```bash
make clean
```

Removes `binaries/sc-*` and runs `cargo clean` / `go clean` in gin and axum.
