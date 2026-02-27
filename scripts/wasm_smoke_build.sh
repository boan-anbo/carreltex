#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: cargo not found" >&2
  exit 2
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo "ERROR: rustup not found; install rustup and add wasm32-unknown-unknown target" >&2
  exit 2
fi

if ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  echo "ERROR: rust target wasm32-unknown-unknown not installed" >&2
  echo "HINT: rustup target add wasm32-unknown-unknown" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cargo build \
  --manifest-path "$ROOT_DIR/crates/carreltex-wasm-smoke/Cargo.toml" \
  --target wasm32-unknown-unknown
