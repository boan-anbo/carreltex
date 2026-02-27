# CarrelTeX Progress Ledger

Allowed status enum: `todo | stubbed | implemented | verified | skipped`.

| path | layer | component | status | proof | notes |
| --- | --- | --- | --- | --- | --- |
| `crates/carreltex-core/src/mount.rs` | core | mount-policy | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Path policy SSOT via `normalize_path_v0` + `read_file_by_bytes_v0`, resource caps, finalize rules, and byte-level (non-UTF8 allowed) main.tex validation |
| `crates/carreltex-core/src/compile.rs` | core | compile-contract-types-v0 | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Compile status/request/result types + canonical report builder/validator + status-token/missing-components helper checks + bounded binary event encoding helpers/constants |
| `crates/carreltex-engine/src/lib.rs` | engine | compile-seam-v0 | verified | `cargo test --manifest-path crates/carreltex-engine/Cargo.toml` | Compile behavior seam; deterministic bounded compile logs + explicit `main.xdv` artifact seam (empty until engine wired) |
| `crates/carreltex-wasm-smoke/src/lib.rs` | wasm-adapter | abi-v0 | verified | `./scripts/proof_v0.sh` | Thin ABI adapter over core+engine semantics, strict report/status+missing_components cross-consistency, per-path log bounds, deterministic binary events seam for log streaming, generic artifact-by-name ABI + `main.xdv` copy-out cap enforcement, and mount read-back ABI |
| `scripts/proof_v0.sh` | proof | v0-bundle | verified | `./scripts/proof_v0.sh` | Bundle gate: core tests + wasm smoke + ledger check |
| `scripts/wasm_smoke_js_proof.mjs` | proof | wasm-js-smoke | verified | `./scripts/proof_wasm_smoke.sh` | JS ABI compatibility checks including compile-request path |
