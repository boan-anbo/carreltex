# CarrelTeX Architecture (Leaf 26)

## Directory map

- `crates/carreltex-core/`
  - Pure Rust logic and contracts.
  - Owns mount/path policy, compile request/result types, and compile contract behavior.
  - Must not depend on WASM/Node-specific APIs.
- `crates/carreltex-wasm-smoke/`
  - Thin WASM ABI adapter for day-one proof flow.
  - Owns ABI exports, in-memory request/report buffers, and translation to/from `carreltex-core`.
  - Must stay thin: semantic rules belong in `carreltex-core`.
- `scripts/`
  - Proof entrypoints and smoke checks.
  - `proof_v0.sh` is the required green pipeline for v0.
- `docs/`
  - Architecture/invariants/ledger source of truth for lane execution.

## Layering rules

- Allowed dependency direction:
  - `carreltex-wasm-smoke -> carreltex-core`
  - `scripts/* -> crates/*`
- Not allowed:
  - `carreltex-core -> carreltex-wasm-smoke`
  - Core semantic rules implemented only in JS (must exist in Rust unit tests).

## Current crate responsibilities

- `crates/carreltex-core`
  - Mount policy (paths, caps, fail-closed validation).
  - Compile request surface and result/report contracts.
- `crates/carreltex-wasm-smoke`
  - ABI compatibility surface (`alloc/dealloc`, mount ABI, compile ABI).
  - Last-report storage and copy-out helpers for JS proof.

## Planned crates (doc-only for now)

- `crates/carreltex-engine` — real XeTeX pipeline.
- `crates/carreltex-wasm` — production WASM ABI.
- `crates/carreltex-fonts` / `crates/carreltex-io` — optional split when engine complexity grows.

## Invariants that shape architecture

- Fail-closed behavior on invalid input or missing prerequisites.
- Determinism knobs must remain explicit (including `SOURCE_DATE_EPOCH` concept).
- Resource caps enforced in core logic, not only at ABI boundary.
