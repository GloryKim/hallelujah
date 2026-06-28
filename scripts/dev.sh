#!/usr/bin/env bash
# Full dev orchestration — starts all sidecars from source, then runs Tauri.
# Kill this script (Ctrl+C) to stop everything.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORTS_YAML="$ROOT/packages/contracts/ports.yaml"

# parse port from ports.yaml without a YAML library dependency
port_of() {
  grep -A3 "^  $1:" "$PORTS_YAML" | grep "port:" | awk '{print $2}'
}

GIN_PORT=$(port_of gin)
EXPRESS_PORT=$(port_of express)
FASTAPI_PORT=$(port_of fastapi)
NEST_PORT=$(port_of nest)

echo "[dev] starting sidecars..."

# Go (Gin)
(cd "$ROOT/services/gin" && SIDECAR_PORT=$GIN_PORT go run ./cmd/server) &
GIN_PID=$!

# Express
SIDECAR_PORT=$EXPRESS_PORT pnpm --filter express dev &
EXPRESS_PID=$!

# FastAPI
SIDECAR_PORT=$FASTAPI_PORT uvicorn app.main:app \
  --host 127.0.0.1 --port "$FASTAPI_PORT" --reload \
  --app-dir "$ROOT/services/fastapi" &
FASTAPI_PID=$!

# NestJS
SIDECAR_PORT=$NEST_PORT pnpm --filter nest-api dev &
NEST_PID=$!

cleanup() {
  echo ""
  echo "[dev] stopping sidecars..."
  kill "$GIN_PID" "$EXPRESS_PID" "$FASTAPI_PID" "$NEST_PID" 2>/dev/null || true
  wait 2>/dev/null || true
  echo "[dev] done"
}
trap cleanup EXIT INT TERM

echo "[dev] waiting for sidecars to warm up..."
sleep 2

echo "[dev] starting Tauri..."
cd "$ROOT/apps/desktop"
pnpm tauri dev
