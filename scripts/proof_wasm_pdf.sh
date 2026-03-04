#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${CARRELTEX_WASM_PDF_OUT_DIR:-$ROOT_DIR/target/wasm_pdf_smoke}"

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

if ! command -v node >/dev/null 2>&1; then
  echo "ERROR: node not found; required for wasm->pdf proof" >&2
  exit 2
fi

node "$ROOT_DIR/scripts/wasm_smoke_js_emit_main_pdf_v0.mjs" "$OUT_DIR" >/dev/null

if [[ ! -s "$OUT_DIR/main.pdf" ]]; then
  echo "ERROR: wasm->pdf produced empty PDF at $OUT_DIR/main.pdf" >&2
  exit 1
fi

echo "PASS: wasm->pdf (preview) -> $OUT_DIR/main.pdf"
