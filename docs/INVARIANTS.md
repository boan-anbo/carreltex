# CarrelTeX Invariants

## Short list (README copyable)

- No silent simplified semantics; out-of-scope must be explicit fail-closed (`INVALID_INPUT` / `NOT_IMPLEMENTED`).
- WASM-from-day-1: always buildable for `wasm32-unknown-unknown`.
- Determinism first: controls (including `SOURCE_DATE_EPOCH`) are explicit and validated.
- Resource caps are mandatory and enforced in core logic.
- Core-first semantics: semantic behavior must be covered by Rust unit tests.
- Proof pipeline must stay green (`scripts/proof_v0.sh`).

## Full list

- No silent simplified semantics.
  - If out of scope, return explicit fail-closed status (`INVALID_INPUT` / `NOT_IMPLEMENTED`) and track it in docs/ledger.
- WASM-from-day-1.
  - The project must stay buildable for `wasm32-unknown-unknown` at all times.
- Determinism first.
  - Determinism controls (including `SOURCE_DATE_EPOCH`) are explicit and validated.
- Resource caps are mandatory.
  - File count, path length, per-file bytes, and total bytes are bounded and enforced in core logic.
- Core-first semantics.
  - Semantic behavior must be covered by Rust unit tests in `carreltex-core`; JS proof alone is not sufficient.
- Proof pipeline must stay green.
  - `scripts/proof_v0.sh` is the baseline gate for v0 changes.
