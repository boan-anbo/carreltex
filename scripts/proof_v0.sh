#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERBOSE="${PROOF_V0_VERBOSE:-${PROOF_VERBOSE:-0}}"

if [[ "${1:-}" == "--verbose" ]]; then
  VERBOSE=1
  shift
fi

if [[ $# -ne 0 ]]; then
  echo "ERROR: usage: $0 [--verbose]" >&2
  exit 2
fi

run_step_quiet() {
  local label="$1"
  local output_mode="$2"
  shift 2
  local log_file
  log_file="$(mktemp)"
  if "$@" >"$log_file" 2>&1; then
    if [[ "$output_mode" == "passthrough_pass_line" ]]; then
      local pass_line
      pass_line="$(awk '/^PASS: / { line=$0 } END { if (line) print line }' "$log_file")"
      if [[ -n "$pass_line" ]]; then
        echo "$pass_line"
      else
        echo "PASS: ${label}"
      fi
    else
      echo "PASS: ${label}"
    fi
  else
    local status="$?"
    echo "FAIL: ${label}" >&2
    cat "$log_file" >&2
    rm -f "$log_file"
    exit "$status"
  fi
  rm -f "$log_file"
}

if [[ "$VERBOSE" == "1" ]]; then
  "$ROOT_DIR/scripts/loc_guard.sh" --verbose
  cargo test --manifest-path "$ROOT_DIR/crates/carreltex-core/Cargo.toml"
  cargo test --manifest-path "$ROOT_DIR/crates/carreltex-engine/Cargo.toml"
  "$ROOT_DIR/scripts/proof_wasm_smoke.sh"
  "$ROOT_DIR/scripts/ledger_check.sh" "$ROOT_DIR/docs/LEDGER.md"
else
  run_step_quiet "loc_guard" "quiet" "$ROOT_DIR/scripts/loc_guard.sh"
  run_step_quiet "core tests" "quiet" cargo test --manifest-path "$ROOT_DIR/crates/carreltex-core/Cargo.toml"
  run_step_quiet "engine tests" "quiet" cargo test --manifest-path "$ROOT_DIR/crates/carreltex-engine/Cargo.toml"
  run_step_quiet "wasm smoke" "passthrough_pass_line" "$ROOT_DIR/scripts/proof_wasm_smoke.sh"
  run_step_quiet "ledger check" "passthrough_pass_line" "$ROOT_DIR/scripts/ledger_check.sh" "$ROOT_DIR/docs/LEDGER.md"
fi

echo "PASS: carreltex v0 proof bundle"
