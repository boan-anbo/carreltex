# CarrelTeX Progress Ledger

Allowed status enum: `todo | stubbed | implemented | verified | skipped`.

| path | layer | component | status | proof | notes |
| --- | --- | --- | --- | --- | --- |
| `crates/carreltex-core/src/mount.rs` | core | mount-policy | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Path policy, resource caps, finalize rules with unit tests |
| `crates/carreltex-core/src/compile.rs` | core | compile-contract-v0 | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Compile request/result contract + canonical report JSON |
| `crates/carreltex-wasm-smoke/src/lib.rs` | wasm-adapter | abi-v0 | verified | `./scripts/proof_v0.sh` | Thin ABI adapter over core semantics, compile report copy-out |
| `scripts/proof_v0.sh` | proof | v0-bundle | verified | `./scripts/proof_v0.sh` | Bundle gate: core tests + wasm smoke + ledger check |
| `scripts/wasm_smoke_js_proof.mjs` | proof | wasm-js-smoke | verified | `./scripts/proof_wasm_smoke.sh` | JS ABI compatibility checks including compile-request path |
