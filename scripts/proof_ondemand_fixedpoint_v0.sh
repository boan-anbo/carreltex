#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/target/ondemand_fixedpoint_v0}"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-1700000000}"
export SOURCE_DATE_EPOCH
export TZ=UTC

node "$ROOT_DIR/scripts/ondemand_fixedpoint_v0_proof.mjs" "$OUT_DIR"

if [[ ! -s "$OUT_DIR/ondemand_fixedpoint_summary.json" ]]; then
  echo "FAIL: expected non-empty $OUT_DIR/ondemand_fixedpoint_summary.json" >&2
  exit 1
fi

node "$ROOT_DIR/scripts/ondemand_fixedpoint_v0_validate.mjs" "$OUT_DIR/ondemand_fixedpoint_summary.json"

echo "PASS: on-demand fixedpoint proof artifacts $OUT_DIR"
echo "PASS: on-demand fixedpoint proof"
