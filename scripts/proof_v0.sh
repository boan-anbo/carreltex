#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo test --manifest-path "$ROOT_DIR/crates/carreltex-core/Cargo.toml"
"$ROOT_DIR/scripts/proof_wasm_smoke.sh"

echo "PASS: carreltex v0 proof bundle"

