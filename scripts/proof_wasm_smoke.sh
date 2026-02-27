#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT_DIR/scripts/wasm_smoke_build.sh"

if ! command -v node >/dev/null 2>&1; then
  echo "ERROR: node not found; required for JS proof" >&2
  exit 2
fi

node "$ROOT_DIR/scripts/wasm_smoke_js_proof.mjs"

