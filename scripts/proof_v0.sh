#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo test --manifest-path "$ROOT_DIR/crates/carreltex-core/Cargo.toml"
cargo test --manifest-path "$ROOT_DIR/crates/carreltex-engine/Cargo.toml"
"$ROOT_DIR/scripts/proof_wasm_smoke.sh"
"$ROOT_DIR/scripts/ledger_check.sh" "$ROOT_DIR/docs/LEDGER.md"

echo "PASS: carreltex v0 proof bundle"
