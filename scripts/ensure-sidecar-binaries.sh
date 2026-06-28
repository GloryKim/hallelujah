#!/usr/bin/env bash
# Creates empty placeholder sidecar binaries so `tauri dev` can compile.
# In dev mode, real servers run from source via scripts/dev.sh — not these files.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_DIR="$ROOT/apps/desktop/src-tauri/binaries"
PORTS_YAML="$ROOT/packages/contracts/ports.yaml"

mkdir -p "$BIN_DIR"

TRIPLE=$(rustc -vV | grep "^host" | awk '{print $2}')
if [[ -z "$TRIPLE" ]]; then
  echo "[ensure-sidecar-binaries] ERROR: could not detect Rust target triple" >&2
  exit 1
fi

while IFS= read -r name; do
  dest="$BIN_DIR/${name}-${TRIPLE}"
  if [[ ! -f "$dest" ]]; then
    : > "$dest"
    chmod +x "$dest"
    echo "[ensure-sidecar-binaries] created placeholder $dest"
  fi
done < <(grep "binaryName:" "$PORTS_YAML" | awk '{print $2}')
