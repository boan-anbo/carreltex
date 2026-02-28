#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MAX_LOC=1000
VERBOSE="${LOC_GUARD_VERBOSE:-0}"

if [[ "${1:-}" == "--verbose" ]]; then
  VERBOSE=1
  shift
fi

if [[ $# -ne 0 ]]; then
  echo "ERROR: usage: $0 [--verbose]" >&2
  exit 2
fi

checked=0
violations=0
violation_lines=()

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
    violations=$((violations + 1))
    violation_lines+=("missing file: $rel")
    continue
  fi
  checked=$((checked + 1))
  loc=$(wc -l < "$path")
  if (( loc > MAX_LOC )); then
    violations=$((violations + 1))
    violation_lines+=("$rel lines=$loc limit=$MAX_LOC")
  elif [[ "$VERBOSE" == "1" ]]; then
    echo "PASS: loc_guard $rel lines=$loc limit=$MAX_LOC"
  fi
done < <(cd "$ROOT_DIR" && git ls-files)

if (( checked == 0 )); then
  echo "FAIL: loc_guard no tracked source files matched"
  exit 1
fi

if (( violations != 0 )); then
  echo "FAIL: loc_guard"
  for line in "${violation_lines[@]}"; do
    echo "$line"
  done
  exit 1
fi

echo "PASS: loc_guard"
