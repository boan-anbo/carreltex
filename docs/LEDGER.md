# CarrelTeX Progress Ledger

Allowed status enum: `todo | stubbed | implemented | verified | skipped`.

| path | layer | component | status | proof | notes |
| --- | --- | --- | --- | --- | --- |
| `crates/carreltex-core/src/mount.rs` | core | mount-policy | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Path policy, resource caps, finalize rules, and byte-level (non-UTF8 allowed) main.tex validation |
| `crates/carreltex-core/src/compile.rs` | core | compile-contract-types-v0 | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Compile status/request/result types + canonical report builder/validator |
| `crates/carreltex-engine/src/lib.rs` | engine | compile-seam-v0 | verified | `cargo test --manifest-path crates/carreltex-engine/Cargo.toml` | Compile behavior seam; validates mount/request and returns fail-closed status |
| `crates/carreltex-wasm-smoke/src/lib.rs` | wasm-adapter | abi-v0 | verified | `./scripts/proof_v0.sh` | Thin ABI adapter over core+engine semantics, compile report copy-out |
| `scripts/proof_v0.sh` | proof | v0-bundle | verified | `./scripts/proof_v0.sh` | Bundle gate: core tests + wasm smoke + ledger check |
| `scripts/wasm_smoke_js_proof.mjs` | proof | wasm-js-smoke | verified | `./scripts/proof_wasm_smoke.sh` | JS ABI compatibility checks including compile-request path |
