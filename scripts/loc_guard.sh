#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MAX_LOC=1000

status=0
checked=0

while IFS= read -r rel; do
  if [[ "$rel" == third_party/* || "$rel" == target/* ]]; then
    continue
  fi
  if [[ "$rel" == crates/* && "$rel" == *.rs ]]; then
    :
  elif [[ "$rel" == scripts/* && "$rel" == *.mjs ]]; then
    :
  else
    continue
  fi

  path="$ROOT_DIR/$rel"
  if [[ ! -f "$path" ]]; then
    echo "FAIL: loc_guard missing file: $rel"
    status=1
    continue
  fi
  checked=$((checked + 1))
  loc=$(wc -l < "$path")
  if (( loc > MAX_LOC )); then
    echo "FAIL: loc_guard $rel lines=$loc limit=$MAX_LOC"
    status=1
  else
    echo "PASS: loc_guard $rel lines=$loc limit=$MAX_LOC"
  fi
done < <(cd "$ROOT_DIR" && git ls-files)

if (( checked == 0 )); then
  echo "FAIL: loc_guard no tracked source files matched"
  exit 1
fi

if (( status != 0 )); then
  echo "FAIL: loc_guard"
  exit 1
fi

echo "PASS: loc_guard"
