# Changelog

All notable changes to GVIBU are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- 58 Unix commands fully implemented (spec + Rust + Python reference)
- Cross-platform support: Linux binary, WASM browser demo, Docker image
- Man page generation for all 58 commands (`tooling/generate_manpages.py`)
- Automated command status tracking (`tooling/generate_command_status.py`)
- Shared integration tests in `shared-tests/tests/`
- GitHub Actions CI/CD: parity checks, release builds, WASM deployment, Docker

### Changed
- Fixed stale command status documentation (previously listed only 9 MVP commands)
- Fixed man page generator for aliased commands (e.g. `test`/`[`)
- Fixed WASM demo metadata to reflect actual command count

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
