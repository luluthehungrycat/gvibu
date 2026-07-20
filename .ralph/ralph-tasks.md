# Ralph Tasks

Add your tasks below using: `ralph --add-task "description"`

## Completed (Wave 1–4 of codebase-audit-roadmap)

- [x] Replace production `.unwrap()` calls in chmod.rs, date.rs, du.rs
- [x] Add `// SAFETY:` documentation above every `unsafe` block in id.rs,
      kill.rs, uname.rs, uptime.rs, chown.rs, df.rs
- [x] Replace `.expect(...)` with `.unwrap_or_else(|| unreachable!(...))` in
      expr.rs and sort.rs
- [x] Replace O(n) linear command lookup with O(1) `HashMap` lookup in
      `rust/src/commands/mod.rs` via `LazyLock<HashMap<&'static str,
      &'static Command>>`
- [x] Consolidate rand 0.8 → 0.9 in rust/Cargo.toml and wasm-lib/Cargo.lock
- [x] Add 4 CI jobs to `.github/workflows/parity.yml` (clippy, audit,
      benchmark, fuzz)
- [x] Implement WASM stdin support via `OnceLock<String>` dispatch
      (10 stdin-consuming commands migrated)
- [x] Author `docs/vish-design.md` with REPL spec, built-ins, dispatch
      semantics, and test migration plan
- [x] Build Rust `vish` crate (REPL + subcommand + 5 built-ins) depending
      on the gvibu library
- [x] Rewrite `gvibu-python/vish.py` as a full REPL (146 pure LOC) with
      dynamic command loading and 5 built-ins
- [x] Update `gvibu-python/Dockerfile` (REPL default) and rewrite
      `gvibu-rust/Dockerfile` (multi-stage build of vish)
- [x] Migrate `tests/test_vish_cli.py`, `tests/test_vish_py.py`, and
      `tests/test_vish_integration_py.py` to the new vish design
- [x] Replace echo-only QEMU scaffolds with real `qemu_run_py.sh` and
      `qemu_run_rs.sh` that build variant initramfs and exec run_qemu.sh
- [x] Document Linux userspace testing path in README

## Open

- [ ] Resolve the libc-on-wasm32 issue (gate libc behind
      `cfg(not(target_arch = "wasm32"))` and provide wasm32 stubs for
      chown, df, id, kill, uname, uptime, who) so the browser WASM build
      produces a clean `wasm-pack` output.
- [ ] Add vish scripting (variables, conditionals, pipes, redirects,
      job control) on top of the command-dispatch baseline.
- [ ] Add persistent command history to vish REPL.
- [ ] Wire vish to GVIBU command surface with metadata (descriptions,
      man-page summaries) for `help <command>`.
- [ ] Cross-language vish parity test against GNU coreutils in CI.
- [ ] Extend fuzz harness to cover command argument parsers.
- [ ] Remove the duplicate `gvibu-python/gvibu.py` once `python-ref` is
      the sole Python surface.
