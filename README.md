# CarrelTeX

Browser-first WASM LaTeX/typesetting engine (WIP).

## Hard invariants (v0)

- Always buildable for `wasm32-unknown-unknown` (WASM viability gate).
- Fail-closed by default.
- No silent “simplified” semantics.
- Determinism is a first-class constraint (`SOURCE_DATE_EPOCH`, pinned toolchains).

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
