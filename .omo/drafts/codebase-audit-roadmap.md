---
slug: codebase-audit-roadmap
status: approved
intent: unclear
  review_required: true
  vish_upstream: github.com/luluthehungrycat/vish
  plan_path: .omo/plans/codebase-audit-roadmap.md
  plan_sha256: f2a220fc48a6496cbc9aa77858ecdf3366e0908b0904332c2f9393b0ada5356b
  review_round_id: r2
  pending-action: review .omo/plans/codebase-audit-roadmap.md (round 2 — fixed duplicate numbering, test counts, vish migration, WASM contradiction)
  review:
    momus:
      status: completed_r1
      result: CHANGES_REQUESTED
    independent:
      status: completed_r1
      result: CHANGES_REQUESTED
  r1_issues_fixed:
    - "Duplicate Todo 10 numbering fixed (design→11, shifted +1 through Wave 5)"
    - "chmod.rs:69 removed from Todo 3 (was double-counted with Todo 1)"
    - "Test count corrected: 264→255 CLI tests (actual grep count)"
    - "expr.rs:39 reference fixed (marked as method definition, not panicking call)"
    - "vish test migration plan added to Todo 11 design doc"
    - "Wave 2 independence claim clarified (depends on Todo 5 toolchain)"
    - "Dependency matrix updated to match inline Blocked-by fields"
    - "WASM stdin contradiction resolved (dispatch-level injection, no signature changes)"
    - "'12 production' → '8 production' in Must Have"
approach: >
  The core 58 Unix commands are fully implemented and production-ready with
  comprehensive tests (264 CLI + 35 fuzz + 58 Python). The main gaps are in the
  GVIBU surface layer (gvibu-rust, gvibu-python, vish - all placeholders),
  outdated dependencies, and missing CI coverage (clippy, audit, benchmarks).
  The plan addresses these in priority order: security/deps first, then
  GVIBU surface implementation, then developer experience improvements.
  High-accuracy review is auto-required because intent is UNCLEAR and work is
  non-Trivial.
---

# Draft: codebase-audit-roadmap

## Components (topology ledger)
<!-- Lock the SHAPE before depth. One row per top-level component that can succeed or fail independently. -->
<!-- id | outcome (one line) | status: active|deferred | evidence path -->

| C1 | Fix 30 unwrap() calls in command implementations | active | rust/src/commands/*.rs |
| C2 | Update all outdated Rust dependencies | active | rust/Cargo.toml, wasm-lib/Cargo.toml |
| C3 | Add missing CI checks (clippy, cargo-audit, benchmark, fuzz) | active | .github/workflows/parity.yml |
| C4 | Implement Rust GVIBU surface (gvibu-rust, vish) | active | gvibu-rust/src/, vish/ |
| C5 | Implement Python GVIBU surface (gvibu-python, vish) | active | gvibu-python/, gvibu.py |
| C6 | Address WASM stdin limitation | active | wasm-lib/src/lib.rs |
| C7 | Add QEMU run scripts for Python and Rust GVIBU images | active | gvibu-linux/ |

## Open assumptions (announced defaults)
<!-- Intent is UNCLEAR: research resolves ambiguity, defaults are adopted (not asked), and each is surfaced in the plan's human TL;DR for veto. -->
<!-- assumption | adopted default | rationale | reversible? -->

| A1 | Unwrap cleanup: convert to proper Result propagation with `?` operator, using existing error patterns | Best practice for Rust; most unwraps are in test code already | Yes - could use anyhow or thiserror instead |
| A2 | Dependency updating: `cargo update` with semver-compatible bumps | Safe default; breaking changes unlikely in patch versions | Yes - could pin specific versions |
| A3 | GVIBU surface: implement as a shell REPL dispatching to the 58 commands | Natural extension of existing multicall architecture | Yes - could be a different UX |
| A4 | CI additions: non-blocking initially (allow-failure), then enforcing | Avoids blocking existing workflows | Yes - could go straight to blocking |
| A5 | QEMU run scripts: reuse existing gvibu-linux/build_initramfs.sh pattern for both Rust and Python | Codebase convention | Yes |

## Findings (cited - path:lines)

### F1: Core Commands - Complete but Quality Issues (109 items)
- **Source**: Explorer bg_790aaa32 + direct grep
- All 58 README-listed commands have Rust implementations under `rust/src/commands/`
- Consistent interface: `pub fn run(w: &mut dyn Write, args: &[String]) -> i32`
- Command lookup is O(n) in `rust/src/commands/mod.rs:131-133`
- Quality issues: 30 `.unwrap()`, 9 `.expect()`, 2 `panic!`, 18 `unsafe` across 14 files
- Top offenders: `expr.rs` (646 lines, 15 issues), `sort.rs` (514 lines, 7 issues), `id.rs` (6 unsafe)
- Many unwraps are in test code; production unwraps exist in `cp.rs:275`, `date.rs:19,40`, `du.rs:63`

### F2: Test Suite - Strong Coverage (264+35+58 tests)
- **Source**: Explorer bg_0fc54da5 + direct grep
- `rust/tests/cli.rs`: 2320 lines, 264 test functions covering all 58 commands
- `rust/tests/fuzz.rs`: 525 lines, 35 property-based fuzz tests for 20+ commands
- `shared-tests/cases/`: 58 JSON test case files (one per command)
- `tests/`: 58 Python pytest files (one per command)
- Zero commands lack test coverage
- Commands with no fuzz tests: many text/sorting commands (grep, sort, join, comm, etc.)

### F3: Technical Debt - GVIBU Surface is All Placeholders
- **Source**: Explorer bg_b94d195c + direct read
- Core command files: NO TODO/FIXME/HACK/XXX/BUG markers found — complete
- `gvibu-rust/src/gvibu.rs:8-10`: Placeholder `init`, `status`, `run` returning static strings
- `gvibu-rust/src/main.rs:6-8,26`: Same placeholders, main outputs "placeholder"
- `gvibu-python/gvibu.py:6-10`: All handlers are placeholders
- `gvibu-python/vish.py:12-19`: argparse placeholders for init/status/gvibu subcommands
- `.ralph/ralph-tasks.md`: 13 task items (5 in-progress, 8 pending) for GVIBU surface

### F4: CI/CD - Comprehensive but Missing Linting/Security
- **Source**: Explorer bg_b760ef2e
- `.github/workflows/parity.yml` (97 lines): test, wasm, docker jobs — all passing
- `.github/workflows/release.yml` (162 lines): binary, WASM, Docker, GitHub Release
- Missing: clippy linting, cargo-audit security scanning, benchmark CI, fuzz CI
- No coverage reporting (codecov, tarpaulin)
- Docker: 6 Dockerfiles across root, rust/, python-ref/, gvibu-linux/, gvibu-python/, gvibu-rust/

### F5: Dependencies - All Outdated
- **Source**: Explorer bg_b760ef2e + Cargo.toml reads
- `rust/Cargo.toml`: libc 0.2, rand 0.8, regex 1, chrono 0.4, proptest 1 — all outdated
- `wasm-lib/Cargo.toml`: wasm-bindgen 0.2 — outdated
- `Cargo.toml` (root): clap 4, assert_cmd 2, predicates 1 — outdated
- Lockfile shows mismatched versions (rand 0.8.6 vs available 0.9.4)

### F6: WASM - Functional but Limited
- **Source**: Direct read of `wasm-lib/src/lib.rs`
- Browser WASM works for non-filesystem commands (echo, true, false, pwd, etc.)
- `wasm-lib/src/lib.rs:35-37`: stdin is unused; commands reading stdin panic in WASM
- No WASM tests in CI beyond basic wasmtime smoke tests

### F7: Auxiliary Systems - Mixed State
- **Source**: Explorer bg_79595b4e + direct reads
- `vibix-lib/`: Active — flat binary userspace with Makefile, NASM assembly, 8 VIBIX commands
- `vish/`: Stub — only Cargo.toml, no source files
- `gvibu-linux/`: Active — initramfs/QEMU scripts work, but needs Python/Rust scripts
- `specs/commands/`: Complete — 58 spec files matching all commands
- `man/`: Present via Makefile generation target
- `docs/`: Present but not deeply explored
- `patches/`: Present but not explored
- `handoffs/`: Present but not explored
- `opencode-container/`: docker-compose.yaml is empty (0 bytes)

### F8: Ralph Tasks - Explicit Work Queue
- **Source**: Direct read of `.ralph/ralph-tasks.md`
- 5 tasks marked in-progress (`[/]`), 8 tasks marked pending (`[ ]`)
- All tasks are GVIBU surface related: vish shell, QEMU, Docker, CLI tests, README
- Core commands are NOT in the task list — they are considered done

## Decisions (with rationale)

1. **Prioritize security/deps over GVIBU surface**: Dependencies with known CVEs are a real risk; GVIBU surface is placeholders that don't hurt anyone yet. Cargo-audit and dep updates come first.

2. **unwrap cleanup is scoped to production code only**: Most unwraps are in test code where they're acceptable. Only fix the 12 production-site unwraps across cp, date, du, expr, df, chmod.

3. **GVIBU surface uses the existing 58-command dispatch**: The vish/GVIBU shell is a thin REPL layer, not a reimplementation. Reuse `rust/src/commands/mod.rs` COMMANDS array.

4. **CI additions are non-blocking initially**: Add clippy/audit as `continue-on-error: true` for one month, then enforce. Avoids sudden CI breakage.

5. **WASM stdin: deferred to post-GVIBU**: WASM stdin is a niche use case. The GVIBU surface is higher priority.

## Scope IN

- Fix production `.unwrap()` calls in command implementations
- Update Rust dependencies to latest semver-compatible versions
- Add clippy, cargo-audit, benchmark, and fuzz jobs to CI
- Implement Rust GVIBU surface (gvibu-rust with vish shell, command dispatch)
- Implement Python GVIBU surface (gvibu-python with vish shell, command dispatch)
- Add CLI tests for vish
- Add QEMU run scripts for Python and Rust GVIBU images
- Update README with Linux userspace testing instructions
- WASM stdin support (lower priority, deferred to wave 3)

## Scope OUT (Must NOT have)

- New coreutils commands beyond the existing 58
- Complete rewrite of any existing command
- Changing the public API (run function signature)
- WASM filesystem emulation
- Kernel modifications
- Performance optimization of existing commands (not needed)
- Test framework migration
- GUI or TUI beyond the vish REPL

## Open questions

None — all researchable questions have been answered by the explorations.
Intent is UNCLEAR: defaults are adopted and surfaced below for user veto.

## Approval gate
status: awaiting-approval
