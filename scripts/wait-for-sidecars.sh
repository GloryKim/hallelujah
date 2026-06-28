#!/usr/bin/env bash
# Waits until every sidecar in ports.yaml responds on GET /health.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORTS_YAML="$ROOT/packages/contracts/ports.yaml"
TIMEOUT="${SIDECAR_WAIT_TIMEOUT:-60}"

port_of() {
  grep -A3 "^  $1:" "$PORTS_YAML" | grep "port:" | awk '{print $2}'
}

health_path_of() {
  grep -A4 "^  $1:" "$PORTS_YAML" | grep "healthPath:" | awk '{print $2}'
}

services=(gin express fastapi nest axum)
deadline=$((SECONDS + TIMEOUT))

echo "[dev] waiting for sidecars (timeout ${TIMEOUT}s)..."

while (( SECONDS < deadline )); do
  all_ready=true
  for svc in "${services[@]}"; do
    port=$(port_of "$svc")
    path=$(health_path_of "$svc")
    if ! curl -sf --max-time 1 "http://127.0.0.1:${port}${path}" >/dev/null; then
      all_ready=false
      break
    fi
  done

  if $all_ready; then
    echo "[dev] all sidecars ready"
    exit 0
  fi

  sleep 0.5
done

echo "[dev] ERROR: timed out waiting for sidecars" >&2
for svc in "${services[@]}"; do
  port=$(port_of "$svc")
  path=$(health_path_of "$svc")
  if curl -sf --max-time 1 "http://127.0.0.1:${port}${path}" >/dev/null; then
    echo "  ok  $svc (:$port)"
  else
    echo "  DOWN $svc (:$port)"
  fi
done
exit 1
