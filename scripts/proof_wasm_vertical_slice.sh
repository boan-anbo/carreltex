#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/out}"

"$ROOT_DIR/scripts/wasm_vertical_slice_xdv.sh" "$OUT_DIR"
"$ROOT_DIR/scripts/wasm_vertical_slice_host_preview.sh" "$OUT_DIR"

echo "PASS: wasm vertical slice proof"
