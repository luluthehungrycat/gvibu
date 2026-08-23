# Changelog

All notable changes to GVIBU are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- 60 Unix commands implemented with specs, Rust, and Python reference
- Cross-platform support: Linux binary, WASM browser demo, Docker image
- Man page generation for all 60 commands (`tooling/generate_manpages.py`)
- Automated command status tracking (`tooling/generate_command_status.py`)
- Shared integration tests in `shared-tests/tests/`
- GitHub Actions CI/CD: parity checks, release builds, WASM deployment, Docker
- Shared VIBIX runtime contract in `vibix-lib/src/runtime.rs` with typed
  errno/sentinel conversion, descriptors, checked writes, and a checked `brk`
  boundary
- Maintained GVIBU/VIBIX integration roadmap with command artifact manifest,
  VIBIT handoff scenarios, and image ownership boundaries

### Changed
- Fixed stale command status documentation (previously listed only 9 MVP commands)
- Fixed man page generator for aliased commands (e.g. `test`/`[`)
- Fixed WASM demo metadata to reflect actual command count
- Corrected VIBIX inline-assembly wrapper clobber declarations and raw syscall
  numbering for `open`/`close`
- Migrated VIBIX `echo` output to checked runtime writes while preserving its
  flags, successful output, stderr separation, and exit semantics
- Documented image integration as a separate owner from the VIBIX kernel repo;
  VIBIX remains responsible for kernel/ABI smoke targets only
- Preserved the `test`/`[` alias marker through Rust and Python multicall
  dispatch so missing closing brackets return status `1` consistently
- Corrected Rust `du` diagnostics to stderr, accepted squeeze-only `tr`, and
  rejected invalid `uptime` operands
- Added `rev` and `expand` in Python and Rust with specs and shared parity cases
- Updated parity tooling for legacy case schemas and both command registries

### Verification
- `cargo test --manifest-path rust/Cargo.toml --test cli`: 259 passed
- Focused Python command tests: 103 passed
- Full Python suite: 386 passed
- New `rev` and `expand` Rust/Python tests pass
- `make compare`: 280 of 280 cases pass
- `python tooling/generate_command_status.py` reports all 60 commands complete
- `git diff --check` passes

## [0.1.0] — 2026-06-22

### Added
- Initial MVP with 9 core commands: true, false, echo, pwd, basename, dirname, cat, head, wc
- Rust multicall binary dispatch
- Python reference implementation for parity testing
- Docker and QEMU deployment support
- Basic test suite
- VIBIX flat binary userspace support (8 commands ported)

### Commands added (in build-up order)
- Phase 1 (9 MVP): true, false, echo, pwd, basename, dirname, cat, head, wc
- Phase 2: yes, printenv, sleep, touch, seq, which, uname, env, whoami, link, unlink, tee, mkdir, rmdir
- Phase 3: hostname, logname, readlink, realpath, uniq, uptime, id, who, kill, cut, tr, mv, rm, ln, chmod, chown, sort, test
- Phase 4: grep, ls, tail, tac, fold, comm, join, nl, shuf, sum, cp, printf, date, expr, split, du, df
