#!/usr/bin/env bash
# Packages the FastAPI app into a standalone binary via PyInstaller,
# then copies it with the Tauri target-triple suffix into binaries/.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(dirname "$SCRIPT_DIR")"
ROOT_DIR="$(dirname "$(dirname "$SERVICE_DIR")")"
BIN_DIR="$ROOT_DIR/apps/desktop/src-tauri/binaries"

mkdir -p "$BIN_DIR"

cd "$SERVICE_DIR"

# build a single-file executable
pyinstaller \
  --onefile \
  --name sc-fastapi \
  --distpath dist-bin \
  app/main.py

# detect Tauri target triple
TRIPLE=$(rustc -vV | grep "^host" | awk '{print $2}')
if [[ -z "$TRIPLE" ]]; then
  echo "[build-sidecar] ERROR: could not detect Rust target triple" >&2
  exit 1
fi

SRC="$SERVICE_DIR/dist-bin/sc-fastapi"
DEST="$BIN_DIR/sc-fastapi-$TRIPLE"

cp "$SRC" "$DEST"
echo "[build-sidecar] copied sc-fastapi → $DEST"
