#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/out}"
XDV_PATH="$OUT_DIR/main.xdv"
PDF_PATH="$OUT_DIR/main.pdf"
MAP_PATH="$OUT_DIR/carreltex-v0.map"
TFM_ALIAS_PATH="$OUT_DIR/carreltex-v0.tfm"

if [[ ! -s "$XDV_PATH" ]]; then
  echo "FAIL: missing xdv artifact $XDV_PATH" >&2
  echo "HINT: run ./scripts/wasm_vertical_slice_xdv.sh first" >&2
  exit 1
fi

if ! command -v xdvipdfmx >/dev/null 2>&1; then
  echo "FAIL: xdvipdfmx not found (host preview unavailable)" >&2
  exit 2
fi

if ! command -v kpsewhich >/dev/null 2>&1; then
  echo "FAIL: kpsewhich not found (cannot prepare host preview font alias)" >&2
  exit 2
fi

CMR10_TFM="$(kpsewhich cmr10.tfm || true)"
if [[ -z "$CMR10_TFM" || ! -f "$CMR10_TFM" ]]; then
  echo "FAIL: cmr10.tfm not found via kpsewhich" >&2
  exit 1
fi
cp "$CMR10_TFM" "$TFM_ALIAS_PATH"

cat >"$MAP_PATH" <<'EOF'
carreltex-v0 CMR10 <cmr10.pfb
EOF

TEXFONTS="$OUT_DIR:" xdvipdfmx -f "$MAP_PATH" -o "$PDF_PATH" "$XDV_PATH" >/dev/null

if [[ ! -s "$PDF_PATH" ]]; then
  echo "FAIL: expected non-empty $PDF_PATH" >&2
  exit 1
fi

echo "PASS: host preview pdf artifact $PDF_PATH"
