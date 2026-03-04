#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${CARRELTEX_WASM_DEMO_OUT_DIR:-$ROOT_DIR/target/wasm_pdf_demo}"
FIXTURE_PATH="${CARRELTEX_WASM_DEMO_TEX:-$ROOT_DIR/scripts/wasm_smoke_js/fixtures/ok_demo_v0.tex}"

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

if ! command -v node >/dev/null 2>&1; then
  echo "ERROR: node not found; required for wasm demo pdf" >&2
  exit 2
fi

node "$ROOT_DIR/scripts/wasm_smoke_js_emit_main_pdf_v0.mjs" "$OUT_DIR" "$FIXTURE_PATH" >/dev/null

if [[ ! -s "$OUT_DIR/main.xdv" ]]; then
  echo "ERROR: demo did not produce XDV at $OUT_DIR/main.xdv" >&2
  exit 1
fi

if [[ ! -s "$OUT_DIR/main.pdf" ]]; then
  echo "ERROR: demo did not produce PDF at $OUT_DIR/main.pdf" >&2
  exit 1
fi

if [[ "${CARRELTEX_OPEN_PDF:-0}" == "1" ]] && command -v open >/dev/null 2>&1; then
  open "$OUT_DIR/main.pdf"
fi

echo "PASS: wasm demo -> $OUT_DIR/main.pdf (preview)"
