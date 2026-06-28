#!/usr/bin/env bash
# Builds all sidecars and copies them to apps/desktop/src-tauri/binaries/.
# Run this before `pnpm tauri build`.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_DIR="$ROOT/apps/desktop/src-tauri/binaries"

mkdir -p "$BIN_DIR"

echo "[build-sidecars] building gin..."
(cd "$ROOT/services/gin" && make build-sidecar)

echo "[build-sidecars] building express..."
(cd "$ROOT/services/express" && node scripts/build-sidecar.mjs)

echo "[build-sidecars] building fastapi..."
(cd "$ROOT/services/fastapi" && bash scripts/build-sidecar.sh)

echo "[build-sidecars] building nest..."
(cd "$ROOT/services/nest" && node scripts/build-sidecar.mjs)

echo "[build-sidecars] done. Artifacts:"
ls -lh "$BIN_DIR"
