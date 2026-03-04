#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${CARRELTEX_TEXLIVE_DEMO_OUT_DIR:-$ROOT_DIR/target/texlive_pdf_demo}"
FIXTURE_PATH="${CARRELTEX_TEXLIVE_DEMO_TEX:-$ROOT_DIR/scripts/texlive_smoke/fixtures/typeset_demo_capabilities_v0.tex}"

if ! command -v xelatex >/dev/null 2>&1; then
  echo "ERROR: xelatex not found (TeX Live required)" >&2
  exit 2
fi

if ! command -v xdvipdfmx >/dev/null 2>&1; then
  echo "ERROR: xdvipdfmx not found (TeX Live required)" >&2
  exit 2
fi

mkdir -p "$OUT_DIR"

# Determinism knobs (best-effort; depends on TeX Live + driver behavior).
export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export TZ="${TZ:-UTC}"

# Phase 1 (v1 invariant): XeTeX -> XDV
xelatex \
  -halt-on-error \
  -interaction=nonstopmode \
  -no-pdf \
  -jobname=main \
  -output-comment="CarrelTeX demo" \
  -output-directory="$OUT_DIR" \
  "$FIXTURE_PATH" \
  >/dev/null

if [[ ! -s "$OUT_DIR/main.xdv" ]]; then
  echo "ERROR: expected XDV at $OUT_DIR/main.xdv" >&2
  exit 1
fi

# Phase 2 (v2+): XDV -> PDF (native TeX Live driver; not the browser/WASM path).
xdvipdfmx -q -o "$OUT_DIR/main.pdf" "$OUT_DIR/main.xdv" >/dev/null

if [[ ! -s "$OUT_DIR/main.pdf" ]]; then
  echo "ERROR: expected PDF at $OUT_DIR/main.pdf" >&2
  exit 1
fi

if [[ "${CARRELTEX_OPEN_PDF:-0}" == "1" ]] && command -v open >/dev/null 2>&1; then
  open "$OUT_DIR/main.pdf"
fi

echo "PASS: texlive demo -> $OUT_DIR/main.pdf (typeset; XDV primary)"
