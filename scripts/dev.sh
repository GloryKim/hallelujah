#!/usr/bin/env bash
# Full dev orchestration — starts all sidecars from source, then runs Tauri.
# Kill this script (Ctrl+C) to stop everything.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORTS_YAML="$ROOT/packages/contracts/ports.yaml"

# Tauri requires externalBin files to exist at compile time.
# Real servers run from source below; placeholders satisfy the build check.
"$ROOT/scripts/ensure-sidecar-binaries.sh"

# parse port from ports.yaml without a YAML library dependency
port_of() {
  grep -A3 "^  $1:" "$PORTS_YAML" | grep "port:" | awk '{print $2}'
}

GIN_PORT=$(port_of gin)
EXPRESS_PORT=$(port_of express)
FASTAPI_PORT=$(port_of fastapi)
NEST_PORT=$(port_of nest)
AXUM_PORT=$(port_of axum)

# Drop stale listeners so updated source builds (with CORS) can bind.
"$ROOT/scripts/free-sidecar-ports.sh"

echo "[dev] starting sidecars..."

# Go (Gin)
(cd "$ROOT/services/gin" && SIDECAR_PORT=$GIN_PORT go run ./cmd/server) &
GIN_PID=$!

# Express
SIDECAR_PORT=$EXPRESS_PORT pnpm --filter express dev &
EXPRESS_PID=$!

# FastAPI — use python -m uvicorn (uvicorn is often not on PATH)
if ! python3 -c "import uvicorn" 2>/dev/null; then
  echo "[dev] installing fastapi Python deps..."
  python3 -m pip install -q -e "$ROOT/services/fastapi"
fi
SIDECAR_PORT=$FASTAPI_PORT python3 -m uvicorn app.main:app \
  --host 127.0.0.1 --port "$FASTAPI_PORT" --reload \
  --app-dir "$ROOT/services/fastapi" &
FASTAPI_PID=$!

# NestJS
SIDECAR_PORT=$NEST_PORT pnpm --filter nest-api dev &
NEST_PID=$!

# Axum
(cd "$ROOT/services/axum" && SIDECAR_PORT=$AXUM_PORT cargo run) &
AXUM_PID=$!

cleanup() {
  echo ""
  echo "[dev] stopping sidecars..."
  kill "$GIN_PID" "$EXPRESS_PID" "$FASTAPI_PID" "$NEST_PID" "$AXUM_PID" 2>/dev/null || true
  wait 2>/dev/null || true
  echo "[dev] done"
}
trap cleanup EXIT INT TERM

"$ROOT/scripts/wait-for-sidecars.sh"

echo "[dev] starting Tauri..."
cd "$ROOT/apps/desktop"
export TAURI_SKIP_SIDECAR_SPAWN=1
pnpm tauri dev
