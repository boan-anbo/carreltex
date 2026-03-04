#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/out}"

"$ROOT_DIR/scripts/wasm_smoke_build.sh"
node "$ROOT_DIR/scripts/wasm_vertical_slice_emit_pdf.mjs" "$OUT_DIR"

if [[ ! -s "$OUT_DIR/main.xdv" ]]; then
  echo "FAIL: expected non-empty $OUT_DIR/main.xdv" >&2
  exit 1
fi

if [[ ! -s "$OUT_DIR/main.pdf" ]]; then
  echo "FAIL: expected non-empty $OUT_DIR/main.pdf" >&2
  exit 1
fi

echo "PASS: wasm vertical slice pdf artifacts $OUT_DIR/main.{xdv,pdf}"

