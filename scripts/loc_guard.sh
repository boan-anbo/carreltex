#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MAX_LOC=1000

FILES=(
  "crates/carreltex-engine/src/lib.rs"
  "crates/carreltex-core/src/compile.rs"
  "crates/carreltex-wasm-smoke/src/lib.rs"
  "scripts/wasm_smoke_js_proof.mjs"
)

status=0
for rel in "${FILES[@]}"; do
  path="$ROOT_DIR/$rel"
  if [[ ! -f "$path" ]]; then
    echo "FAIL: loc_guard missing file: $rel"
    status=1
    continue
  fi
  loc=$(wc -l < "$path")
  if (( loc > MAX_LOC )); then
    echo "FAIL: loc_guard $rel lines=$loc limit=$MAX_LOC"
    status=1
  else
    echo "PASS: loc_guard $rel lines=$loc limit=$MAX_LOC"
  fi
done

if (( status != 0 )); then
  echo "FAIL: loc_guard"
  exit 1
fi

echo "PASS: loc_guard"
