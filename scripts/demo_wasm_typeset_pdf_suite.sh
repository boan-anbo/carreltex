#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_ROOT_DIR="${CARRELTEX_WASM_TYPESET_SUITE_OUT_DIR:-$ROOT_DIR/target/wasm_typeset_suite}"

SWIFTLATEX_TAG="${CARRELTEX_SWIFTLATEX_TAG:-v20022022}"
SWIFTLATEX_ZIP_URL="${CARRELTEX_SWIFTLATEX_ZIP_URL:-https://github.com/SwiftLaTeX/SwiftLaTeX/releases/download/${SWIFTLATEX_TAG}/20-02-2022.zip}"
SWIFTLATEX_DIST_DIR="${CARRELTEX_SWIFTLATEX_DIST_DIR:-$ROOT_DIR/target/third_party/swiftlatex/${SWIFTLATEX_TAG}}"
TEXLIVE_BACKEND="${CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND:-local}"

FIXTURE_A="${CARRELTEX_WASM_TYPESET_SUITE_TEX_A:-$ROOT_DIR/scripts/texlive_smoke/fixtures/typeset_demo_minimal_v0.tex}"
FIXTURE_B="${CARRELTEX_WASM_TYPESET_SUITE_TEX_B:-$ROOT_DIR/scripts/texlive_smoke/fixtures/typeset_demo_capabilities_v0.tex}"

if ! command -v node >/dev/null 2>&1; then
  echo "ERROR: node not found; required for wasm typeset suite" >&2
  exit 2
fi

if ! command -v curl >/dev/null 2>&1; then
  echo "ERROR: curl not found; required to fetch SwiftLaTeX dist + TeX Live blobs" >&2
  exit 2
fi

if ! command -v unzip >/dev/null 2>&1; then
  echo "ERROR: unzip not found; required to unpack SwiftLaTeX dist zip" >&2
  exit 2
fi

if [[ "$TEXLIVE_BACKEND" == "local" ]] && ! command -v kpsewhich >/dev/null 2>&1; then
  echo "ERROR: kpsewhich not found; required for CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND=local" >&2
  echo "Hint: set CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND=remote to force the SwiftLaTeX texlive endpoint." >&2
  exit 2
fi

mkdir -p "$OUT_ROOT_DIR"
mkdir -p "$(dirname "$SWIFTLATEX_DIST_DIR")"

if [[ ! -s "$SWIFTLATEX_DIST_DIR/swiftlatexxetex.wasm" ]] || [[ ! -s "$SWIFTLATEX_DIST_DIR/swiftlatexdvipdfm.wasm" ]]; then
  tmp_zip="$(mktemp -t swiftlatex_dist.XXXXXX.zip)"
  trap 'rm -f "$tmp_zip"' EXIT
  curl -fsSL -o "$tmp_zip" "$SWIFTLATEX_ZIP_URL"
  rm -rf "$SWIFTLATEX_DIST_DIR"
  mkdir -p "$SWIFTLATEX_DIST_DIR"
  unzip -q "$tmp_zip" -d "$SWIFTLATEX_DIST_DIR"
fi

CARRELTEX_SWIFTLATEX_TEXLIVE_BACKEND="$TEXLIVE_BACKEND" \
node "$ROOT_DIR/scripts/wasm_typeset_suite_swiftlatex_v0.mjs" \
  "$TEXLIVE_BACKEND" \
  "$SWIFTLATEX_DIST_DIR" \
  "$OUT_ROOT_DIR" \
  "$FIXTURE_A" \
  "$FIXTURE_B"

for case_dir in "$OUT_ROOT_DIR"/*; do
  if [[ ! -d "$case_dir" ]]; then
    continue
  fi
  if [[ ! -s "$case_dir/main.xdv" ]]; then
    echo "ERROR: missing XDV at $case_dir/main.xdv" >&2
    exit 1
  fi
  if [[ ! -s "$case_dir/main.pdf" ]]; then
    echo "ERROR: missing PDF at $case_dir/main.pdf" >&2
    exit 1
  fi
done

if [[ "${CARRELTEX_OPEN_PDF:-0}" == "1" ]] && command -v open >/dev/null 2>&1; then
  open "$OUT_ROOT_DIR"/*/main.pdf
fi

echo "PASS: wasm typeset suite -> $OUT_ROOT_DIR/*/main.pdf (XDV primary)"
