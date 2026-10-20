# SDD ledger — plan: docs/superpowers/plans/2026-10-04-id-primary-group.md

Pre-flight: no shared interfaces with other plans; this change is independent of VIBIT and VISH.
Ruling: keep group composition helpers pure and pass the selected real/effective primary GID from the option parser — this makes empty-list and duplicate tests deterministic without changing process credentials — cost if wrong: helper may obscure a simpler platform-specific solution.

Implementation and required verification complete. Python: 392 tests passed. Rust: `cargo test --locked --manifest-path rust/Cargo.toml` passed (601 unit, CLI, and fuzz tests). The Python/Rust parity run passed all 266 cases using `GVIBU_RUST_BIN=/workspace/.onboarding/targets/gvibu/debug/gvibu`. The optional `wasm32-wasip1` build was attempted and failed on existing Unix-only APIs; the limitation is documented in `docs/wasm.md`. `wasm-pack` and `wasmtime` are unavailable. Commit `31bd6ea` is pushed on `codex/onboarding-fixes`; draft PR creation awaits GitHub CLI authentication.
