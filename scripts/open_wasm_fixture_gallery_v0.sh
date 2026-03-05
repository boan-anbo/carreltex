#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/wasm_fixture_gallery_v0}"

"$ROOT_DIR/scripts/proof_wasm_fixture_gallery_v0.sh" "$OUT_DIR"

OUT_DIR_ABS="$(cd "$OUT_DIR" && pwd)"

if [[ "$(uname -s)" == "Darwin" ]] && command -v open >/dev/null 2>&1; then
  open -R "$OUT_DIR_ABS" >/dev/null 2>&1 || open "$OUT_DIR_ABS" >/dev/null 2>&1 || true
else
  echo "INFO: gallery directory $OUT_DIR_ABS"
fi

echo "PASS: wasm fixture gallery open $OUT_DIR_ABS"
