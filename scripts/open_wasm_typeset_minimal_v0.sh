#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/out}"
if [[ "$OUT_DIR" != /* ]]; then
  OUT_DIR="$ROOT_DIR/$OUT_DIR"
fi

"$ROOT_DIR/scripts/proof_wasm_typeset_minimal_v0.sh" "$OUT_DIR"

if command -v open >/dev/null 2>&1; then
  open -R "$OUT_DIR/main.pdf" >/dev/null 2>&1 || true
  open -R "$OUT_DIR/main.xdv" >/dev/null 2>&1 || true
else
  echo "INFO: open command not found; artifacts:"
  echo "INFO: $OUT_DIR/main.pdf"
  echo "INFO: $OUT_DIR/main.xdv"
fi

echo "PASS: wasm typeset minimal open artifacts $OUT_DIR/main.xdv $OUT_DIR/main.pdf"
