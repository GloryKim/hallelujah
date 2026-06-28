#!/usr/bin/env bash
# Stops any process listening on registered sidecar ports.
# Prevents stale binaries (without CORS) from blocking dev.sh source startups.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORTS_YAML="$ROOT/packages/contracts/ports.yaml"

kill_port() {
  local port=$1
  local pids
  pids=$(lsof -ti "tcp:${port}" -sTCP:LISTEN 2>/dev/null || true)
  if [[ -n "$pids" ]]; then
    echo "[dev] freeing port ${port} (pid ${pids//$'\n'/, })"
    # shellcheck disable=SC2086
    kill $pids 2>/dev/null || true
  fi
}

while IFS= read -r port; do
  [[ -n "$port" ]] && kill_port "$port"
done < <(grep "port:" "$PORTS_YAML" | awk '{print $2}')

# Give the OS a moment to release sockets.
sleep 0.3
