#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"
STORE_DIR="${OUT_DIR}_store"
REPORT_PATH="$OUT_DIR/report.json"
AUTO_BASELINE="${WASM_FIXTURE_GALLERY_AUTO_BASELINE_PACK_V0:-0}"
BASELINE_PACKS_ROOT="${TEXLIVE_BASELINE_PACKS_DIR_V0:-$ROOT_DIR/target/texlive_smoke/baselines_v0}"
SKIP_PROOF="${WASM_FIXTURE_GALLERY_SKIP_PROOF_V0:-0}"
NO_OPEN="${WASM_FIXTURE_GALLERY_NO_OPEN_V0:-0}"
SELECTED_BASELINE_DIR=""

if [[ "$SKIP_PROOF" != "1" ]]; then
  "$ROOT_DIR/scripts/proof_wasm_fixture_gallery_v0.sh" "$OUT_DIR"
fi

if [[ "$AUTO_BASELINE" == "1" ]]; then
  if [[ ! -s "$REPORT_PATH" ]]; then
    echo "FAIL: expected non-empty report at $REPORT_PATH before baseline auto-select" >&2
    exit 1
  fi
  SELECTED_BASELINE_DIR="$(
    node "$ROOT_DIR/scripts/texlive_smoke/baselines_v0/select_v0.mjs" \
      "$REPORT_PATH" \
      "$BASELINE_PACKS_ROOT"
  )"
  if [[ -z "$SELECTED_BASELINE_DIR" ]]; then
    echo "FAIL: baseline selector returned empty path" >&2
    exit 1
  fi
  TEXLIVE_RESOLVER_BACKEND_V0=offline_store_v0 \
  TEXLIVE_STORE_DIR_V0="$STORE_DIR" \
  TEXLIVE_BASELINE_DIR="$SELECTED_BASELINE_DIR" \
  node "$ROOT_DIR/scripts/wasm_fixture_gallery_v0.mjs" "$OUT_DIR"
fi

OUT_DIR_ABS="$(cd "$OUT_DIR" && pwd)"
REPORT_PATH_ABS="$(cd "$(dirname "$REPORT_PATH")" && pwd)/$(basename "$REPORT_PATH")"

if [[ "$NO_OPEN" == "1" ]]; then
  :
elif [[ "$(uname -s)" == "Darwin" ]] && command -v open >/dev/null 2>&1; then
  open -R "$REPORT_PATH_ABS" >/dev/null 2>&1 || open -R "$OUT_DIR_ABS" >/dev/null 2>&1 || open "$OUT_DIR_ABS" >/dev/null 2>&1 || true
else
  echo "INFO: gallery report $REPORT_PATH_ABS"
  echo "INFO: gallery directory $OUT_DIR_ABS"
fi

if [[ -n "$SELECTED_BASELINE_DIR" ]]; then
  echo "PASS: baseline pack selected $SELECTED_BASELINE_DIR"
fi
echo "PASS: wasm fixture gallery open $OUT_DIR_ABS"
