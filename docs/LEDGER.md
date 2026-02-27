# CarrelTeX Progress Ledger

Allowed status enum: `todo | stubbed | implemented | verified | skipped`.

| path | layer | component | status | proof | notes |
| --- | --- | --- | --- | --- | --- |
| `crates/carreltex-core/src/mount.rs` | core | mount-policy | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Path policy SSOT via `normalize_path_v0` + `read_file_by_bytes_v0`, resource caps, finalize rules, and byte-level (non-UTF8 allowed) main.tex validation |
| `crates/carreltex-core/src/compile.rs` | core | compile-contract-types-v0 | verified | `cargo test --manifest-path crates/carreltex-core/Cargo.toml` | Compile status/request/result types + canonical report builder/validator + strict TeX stats JSON SSOT (`build_tex_stats_json_v0` + `validate_tex_stats_json_v0`) + status-token/missing-components helper checks + bounded binary event encoding helpers/constants (kind=1 log bytes, kind=2 TeX stats JSON) |
| `crates/carreltex-engine/src/lib.rs` | engine | compile-seam-v0 | verified | `cargo test --manifest-path crates/carreltex-engine/Cargo.toml` | Compile behavior seam; tokenizer validation now includes parse-stub group-balance + deterministic token stats JSON produced via core builder and fed into events(kind=2), deterministic bounded compile logs, deterministic INVALID_INPUT reason-token logs (`INVALID_INPUT: ...`), and explicit `main.xdv` artifact seam (empty until engine wired) |
| `crates/carreltex-engine/src/tex/tokenize_v0.rs` | engine | tex-tokenizer-v0 | verified | `cargo test --manifest-path crates/carreltex-engine/Cargo.toml` | Deterministic TeX lexing subset with explicit v0 assumptions (NUL invalid, `%` comments, whitespace coalescing, control words/symbols, token cap fail-closed) |
| `crates/carreltex-wasm-smoke/src/lib.rs` | wasm-adapter | abi-v0 | verified | `./scripts/proof_v0.sh` | Thin ABI adapter over core+engine semantics, strict report/status+missing_components cross-consistency, per-path log bounds + TeX stats JSON invariants with core validator defense-in-depth, deterministic binary events seam carrying kind=1(log bytes)+kind=2(stats JSON), allocator bounded by `MAX_WASM_ALLOC_BYTES_V0` (artifact-aligned), generic artifact-by-name ABI + `main.xdv` copy-out cap enforcement, and mount read-back ABI |
| `scripts/proof_v0.sh` | proof | v0-bundle | verified | `./scripts/proof_v0.sh` | Bundle gate: core tests + wasm smoke + ledger check |
| `scripts/wasm_smoke_js_proof.mjs` | proof | wasm-js-smoke | verified | `./scripts/proof_wasm_smoke.sh` | JS ABI compatibility checks including compile-request path |
