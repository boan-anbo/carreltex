# CarrelTeX

Browser-first WASM LaTeX/typesetting engine (WIP).

## SSOT

- GitHub-canonical SSOT issue: `#22`  
  https://github.com/boan-anbo/carreltex/issues/22

## Hard invariants (v0, short list)

- No silent simplified semantics; out-of-scope returns explicit fail-closed status.
- Always buildable for `wasm32-unknown-unknown` (WASM-from-day-1).
- Determinism first (`SOURCE_DATE_EPOCH` and explicit controls).
- Resource caps are mandatory and enforced in core logic.
- Core-first semantics: Rust unit tests are authoritative for semantic behavior.
- Proof pipeline must stay green on every change.
- Canonical gate command: `./scripts/proof_v0.sh`.

## Docs

- `docs/ARCHITECTURE.md`
- `docs/INVARIANTS.md`
- `docs/LEDGER.md`

## Proof (WASM viability gate)

```bash
scripts/wasm_smoke_build.sh
```

## Proof (JS loads WASM)

```bash
scripts/proof_wasm_smoke.sh
```

## Proof (v0 bundle)

```bash
scripts/proof_v0.sh
```

CI runs the same `scripts/proof_v0.sh` gate on every PR and on pushes to `main`.
