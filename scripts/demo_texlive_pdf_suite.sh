#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_ROOT_DIR="${CARRELTEX_TEXLIVE_SUITE_OUT_DIR:-$ROOT_DIR/target/texlive_pdf_suite}"

FIXTURE_A="${CARRELTEX_TEXLIVE_SUITE_TEX_A:-$ROOT_DIR/scripts/texlive_smoke/fixtures/typeset_demo_minimal_v0.tex}"
FIXTURE_B="${CARRELTEX_TEXLIVE_SUITE_TEX_B:-$ROOT_DIR/scripts/texlive_smoke/fixtures/typeset_demo_capabilities_v0.tex}"

if ! command -v xelatex >/dev/null 2>&1; then
  echo "ERROR: xelatex not found (TeX Live required)" >&2
  exit 2
fi

if ! command -v xdvipdfmx >/dev/null 2>&1; then
  echo "ERROR: xdvipdfmx not found (TeX Live required)" >&2
  exit 2
fi

mkdir -p "$OUT_ROOT_DIR"

# Determinism knobs (best-effort; depends on TeX Live + driver behavior).
export SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export TZ="${TZ:-UTC}"

run_one() {
  local fixture_path="$1"
  local case_name
  case_name="$(basename "$fixture_path" .tex)"
  local out_dir="$OUT_ROOT_DIR/$case_name"

  mkdir -p "$out_dir"

  # Phase 1 (v1 invariant): XeTeX -> XDV
  xelatex \
    -halt-on-error \
    -interaction=nonstopmode \
    -no-pdf \
    -jobname=main \
    -output-comment="CarrelTeX suite demo" \
    -output-directory="$out_dir" \
    "$fixture_path" \
    >/dev/null

  if [[ ! -s "$out_dir/main.xdv" ]]; then
    echo "ERROR: expected XDV at $out_dir/main.xdv" >&2
    exit 1
  fi

  # Phase 2 (v2+): XDV -> PDF (native TeX Live driver; not the browser/WASM path).
  xdvipdfmx -q -o "$out_dir/main.pdf" "$out_dir/main.xdv" >/dev/null

  if [[ ! -s "$out_dir/main.pdf" ]]; then
    echo "ERROR: expected PDF at $out_dir/main.pdf" >&2
    exit 1
  fi

  if [[ "${CARRELTEX_OPEN_PDF:-0}" == "1" ]] && command -v open >/dev/null 2>&1; then
    open "$out_dir/main.pdf"
  fi
}

run_one "$FIXTURE_A"
run_one "$FIXTURE_B"

echo "PASS: texlive suite -> $OUT_ROOT_DIR/*/main.pdf (typeset; XDV primary)"

