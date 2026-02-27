# CarrelTeX

Browser-first WASM LaTeX/typesetting engine (WIP).

## Hard invariants (v0)

- Always buildable for `wasm32-unknown-unknown` (WASM viability gate).
- Fail-closed by default.
- Determinism is a first-class constraint (`SOURCE_DATE_EPOCH`, pinned toolchains).

## Proof (WASM viability gate)

```bash
scripts/wasm_smoke_build.sh
```

## Proof (JS loads WASM)

```bash
scripts/proof_wasm_smoke.sh
```
