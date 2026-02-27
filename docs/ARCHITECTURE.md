# CarrelTeX Architecture (Leaf 26)

## Directory map

- `crates/carreltex-core/`
  - Pure Rust logic and contracts.
  - Owns mount/path policy, compile request/result types, and compile report JSON builder/validation.
  - Must not depend on WASM/Node-specific APIs.
- `crates/carreltex-engine/`
  - Compile pipeline entrypoints over core contracts.
  - Current behavior is explicit fail-closed (`NOT_IMPLEMENTED`) after validation.
- `crates/carreltex-wasm-smoke/`
  - Thin WASM ABI adapter for day-one proof flow.
  - Owns ABI exports, in-memory request/report buffers, and translation to/from core+engine.
  - Must stay thin: semantic rules belong in `carreltex-core` or `carreltex-engine`.
- `scripts/`
  - Proof entrypoints and smoke checks.
  - `proof_v0.sh` is the required green pipeline for v0.
- `docs/`
  - Architecture/invariants/ledger source of truth for lane execution.

## Layering rules

- Allowed dependency direction:
  - `carreltex-engine -> carreltex-core`
  - `carreltex-wasm-smoke -> carreltex-core`
  - `carreltex-wasm-smoke -> carreltex-engine`
  - `scripts/* -> crates/*`
- Not allowed:
  - `carreltex-core -> carreltex-engine`
  - `carreltex-core -> carreltex-wasm-smoke`
  - `carreltex-engine -> carreltex-wasm-smoke`
  - Core semantic rules implemented only in JS (must exist in Rust unit tests).

## Current crate responsibilities

- `crates/carreltex-core`
  - Mount policy (paths, caps, fail-closed validation) and compile contracts/report builder.
- `crates/carreltex-engine`
  - Compile pipeline seam and request-driven compile entrypoints.
- `crates/carreltex-wasm-smoke`
  - ABI compatibility surface (`alloc/dealloc`, mount ABI, compile ABI).
  - Last-report storage and copy-out helpers for JS proof; delegates compile behavior to engine.

## Planned crates (doc-only for now)

- `crates/carreltex-wasm` — production WASM ABI.
- `crates/carreltex-fonts` / `crates/carreltex-io` — optional split when engine complexity grows.

## Invariants that shape architecture

- Fail-closed behavior on invalid input or missing prerequisites.
- Determinism knobs must remain explicit (including `SOURCE_DATE_EPOCH` concept).
- Resource caps enforced in core logic, not only at ABI boundary.
