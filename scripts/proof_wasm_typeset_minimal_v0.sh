#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/out}"

"$ROOT_DIR/scripts/wasm_typeset_minimal_v0.sh" "$OUT_DIR"

echo "PASS: wasm typeset minimal proof"
