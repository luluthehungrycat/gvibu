# codebase-audit-roadmap - Work Plan

## TL;DR (For humans)
<!-- Fill this LAST, after the detailed plan below is written, so it summarizes the REAL plan. -->

**What you'll get:** A production-hardened gvibu with updated dependencies, fixed error handling, CI security/linting coverage, and a working interactive shell (vish) in both Rust and Python that dispatches to all 58 commands.

**Why this approach:** We fix security and quality first (dependencies + unwraps), then lock in quality gates (CI checks), then build the missing GVIBU surface. Each wave is independently verifiable and commit-able.

**What it will NOT do:** Add new commands, rewrite existing commands, change the public API, add a GUI/TUI, emulate a filesystem in WASM, or touch the kernel.

**Effort:** Medium
**Risk:** Low — all changes are additive or patch-level; no API breaks, no kernel changes.
**Decisions I made for you:** Semver-compatible dep bumps, `?` propagation for unwrap cleanup, thin REPL shell reusing existing dispatch, CI checks start as non-blocking, WASM stdin deferred to Wave 4.

Your next move: approve, or run a high-accuracy review. Full execution detail follows below.

---

> TL;DR (machine): Medium effort, Low risk — 5 waves: deps+unwraps (P0), CI quality gates (P1), vish shell Rust+Python (P1), QEMU+WASM+docs (P2), final verification (P3). 30 todos across 5 waves.

## Scope
### Must have
- Fix all 8 production-path `.unwrap()` and `.expect()` calls in command implementations
- Use existing vish repo at `github.com/luluthehungrycat/vish` as upstream — work with that, not a new crate
- Update all Rust dependencies to latest semver-compatible versions (`cargo update`)
- Add clippy linting, `cargo-audit` security scanning, benchmark, and fuzz jobs to CI
- Implement Rust GVIBU surface: a working `vish` binary with REPL dispatching to 58 commands
- Implement Python GVIBU surface: a working `vish` script with REPL dispatching to 58 commands
- Add CLI tests for vish (Rust and Python)
- Add QEMU run scripts for Python and Rust GVIBU images
- Update README with Linux userspace testing instructions

### Must NOT have (guardrails, anti-slop, scope boundaries)
- New coreutils commands beyond the existing 58
- Rewriting or refactoring any existing command module (fixes only)
- Changing the public API signature `pub fn run(w: &mut dyn Write, args: &[String]) -> i32`
- GUI, TUI, or ncurses interface beyond a simple REPL
- WASM filesystem emulation
- Kernel module changes
- Performance optimization of existing commands (not needed)
- Test framework migration (keep proptest, assert_cmd, predicates)

## Verification strategy
> Zero human intervention - all verification is agent-executed.
- Test decision: tests-after — existing test suite validates regression; new tests added for new features
- Evidence: `.omo/evidence/task-<N>-codebase-audit-roadmap.<ext>`

## Execution strategy
### Parallel execution waves
> Target 5-8 todos per wave. Fewer than 3 (except the final) means you under-split.

- **Wave 1** (P0 — security+quality): Fix 6 production unwraps + document unsafe + update deps + toolchain — 6 todos, all parallel
- **Wave 2** (P1 — CI gates): Add clippy, audit, benchmark, fuzz to CI — 4 todos, all parallel. Wave 2 depends on Todo 5 (rust-toolchain.toml) but is otherwise independent of Wave 1 unwrap work — Wave 1 and Wave 2 can run concurrently.
- **Wave 3** (P1 — GVIBU surface): Implement Rust vish + Python vish + CLI tests — 7 todos, Rust and Python parallel
- **Wave 4** (P2 — platform): QEMU scripts, WASM stdin, docs, cleanup — 6 todos, mostly parallel
- **Wave 5** (P3 — final verification): Parity, full test run, build verify — 5 todos, parallel

### Dependency matrix
| Todo | Depends on | Blocks | Can parallelize with |
| --- | --- | --- | --- |
| 1-3 (unwrap/unsafe/expect fixes) | none | none | each other, 4, 5, 6 |
| 4 (dep update + rand) | none | none | 1-3, 5, 6 |
| 5 (rust-toolchain) | none | 7-10 (CI clippy needs pinned version) | 1-4, 6 |
| 6 (HashMap lookup) | none | none | 1-5 |
| 7-10 (CI additions) | 5 | none | each other (all CI YAML changes) |
| 11 (vish design) | none | 12-15 (Rust+Python vish imp.) | none |
| 12-14 (Rust vish implementation) | 11 | 19, 20 | 15-17 |
| 15-17 (Python vish) | 11 | 19, 20 | 12-14 |
| 18 (Rust Docker) | 12 | none | 16, 17 |
| 19-20 (vish CLI tests) | 12, 15 | none | each other |
| 21-26 (platform) | 12, 15 | none | each other |
| 27-31 (final verification) | all above | none | each other |

## Todos
> Implementation + Test = ONE todo. Never separate.
<!-- APPEND TASK BATCHES BELOW THIS LINE WITH edit/apply_patch - never rewrite the headers above. -->

### Wave 1: Security & Quality (P0)

- [x] 1. Fix 6 production unwrap() calls — chmod.rs, date.rs, du.rs (cp.rs unwraps are test-only)
  What to do / Must NOT do: Replace the 6 production `.unwrap()` calls with proper error handling (return 1 + eprintln! to stderr). chmod.rs:69 (chars().next()), date.rs:19 and date.rs:40 (2 unwraps each = 4 total for and_hms_opt + and_local_timezone), du.rs:63 (chars().nth()). The date.rs unwraps are provably safe (0,0,0 always valid for NaiveTime, Utc always succeeds) but should use expect() with message. The cp.rs:275-276 unwraps are in #[cfg(test)] — do NOT touch those. Do NOT change the function signature.
  Parallelization: Wave 1 | Blocked by: none | Blocks: none
  References: rust/src/commands/chmod.rs:69, rust/src/commands/date.rs:19-20, rust/src/commands/date.rs:40-41, rust/src/commands/du.rs:63
  Acceptance criteria (agent-executable): `cargo build --manifest-path rust/Cargo.toml` succeeds with zero errors. `cargo test --manifest-path rust/Cargo.toml` — all existing tests pass. `grep -n '\.unwrap()' rust/src/commands/chmod.rs rust/src/commands/date.rs rust/src/commands/du.rs | grep -v '#\[cfg\(test\)\]' | grep -v 'mod tests'` returns zero hits.
  QA scenarios: happy — `gvibu chmod -x /nonexistent` returns error code not panic; `gvibu date --invalid` exit code 1; `gvibu du /nonexistent` exit code 1. failure — verify no regressions via `cargo test`. Evidence `.omo/evidence/task-1-codebase-audit-roadmap.txt`
  Commit: Y | fix(commands): replace 6 production unwrap calls with error handling

- [x] 2. Fix unsafe blocks — document safety invariants in id.rs, kill.rs, uname.rs, uptime.rs, chown.rs, df.rs
  What to do / Must NOT do: Add `// SAFETY:` comment blocks above each `unsafe { libc::... }` call explaining why the call is safe (valid pointer, correct size, etc.). Do NOT remove the unsafe blocks — they are necessary for libc syscalls.
  Parallelization: Wave 1 | Blocked by: none | Blocks: none
  References: rust/src/commands/id.rs:7,11,15,19,23,28, rust/src/commands/kill.rs:125, rust/src/commands/uname.rs:22-23, rust/src/commands/uptime.rs:18,20, rust/src/commands/chown.rs:71, rust/src/commands/df.rs:288,292,314,318
  Acceptance criteria (agent-executable): Every `unsafe {` block in the listed files has a preceding `// SAFETY:` comment. `cargo build --manifest-path rust/Cargo.toml` succeeds.
  QA scenarios: happy — commands using libc (id, kill, uname, uptime, chown, df) still function correctly. failure — run `cargo clippy` and verify no unsafe-related warnings. Evidence `.omo/evidence/task-2-codebase-audit-roadmap.txt`
  Commit: Y | docs(commands): add SAFETY comments to all unsafe libc blocks

- [x] 3. Fix panic! and expect() calls in production code — expr.rs, sort.rs
  What to do / Must NOT do: Replace production `.expect()` with proper error handling (return 1 + eprintln! to stderr). The real `.expect()` calls are: expr.rs:377 (Option::expect on token lookup) and sort.rs:253 (.expect for sort -k parsing). NOTE: expr.rs:39 is a METHOD DEFINITION (`fn expect(&mut self, ...)` on the Parser struct), NOT a panicking call — do NOT touch it. The self.expect() calls at expr.rs:144-171 use `?` operator and already propagate errors safely. Do NOT touch expects/panics in test code. Do NOT touch chmod.rs:69 (already handled in Todo 1).
  Parallelization: Wave 1 | Blocked by: none | Blocks: none
  References: rust/src/commands/expr.rs:377 (production .expect), rust/src/commands/sort.rs:253 (.expect for sort -k parsing). AVOID: expr.rs:39 (method def, safe), expr.rs:144-171 (custom method with ?, safe)
  Acceptance criteria (agent-executable): `grep -En '\.expect\(' rust/src/commands/{expr,sort}.rs | grep -v '#\[cfg\(test\)\]' | grep -v 'mod tests'` returns zero hits outside test modules. `cargo test --manifest-path rust/Cargo.toml` passes.
  QA scenarios: happy — `gvibu expr 1 + 1` returns "2" (not panic); `gvibu sort -k invalid` returns error. Evidence `.omo/evidence/task-3-codebase-audit-roadmap.txt`
  Commit: Y | fix(commands): replace production expect with error handling in expr, sort

- [x] 4. Fix O(n) command lookup — replace linear search with HashMap in mod.rs
  What to do / Must NOT do: Add `use std::collections::HashMap;` and `use std::sync::LazyLock;` to mod.rs. Create a `LazyLock<HashMap<&str, &Command>>` populated from the COMMANDS array. Update `lookup()` to use the HashMap. Do NOT change the COMMANDS array structure (it's used elsewhere). Keep the array for iteration (wasm-lib/src/lib.rs:67-70 uses it).
  Parallelization: Wave 1 | Blocked by: none | Blocks: none
  References: rust/src/commands/mod.rs:70-133 (COMMANDS array + lookup), wasm-lib/src/lib.rs:67-70 (list_commands iterates COMMANDS)
  Acceptance criteria (agent-executable): `cargo build --manifest-path rust/Cargo.toml` succeeds. `cargo test --manifest-path rust/Cargo.toml` — all 255 CLI tests pass. `cargo test --manifest-path rust/Cargo.toml --test fuzz` — all 35 fuzz tests pass. `gvibu echo hello` prints "hello"; `gvibu nonexistent_command` returns error.
  QA scenarios: happy — all 58 commands still dispatch correctly; WASM `list_commands()` still returns all names. failure — benchmark lookup time for 58 commands (should be near-constant vs. O(n)). Evidence `.omo/evidence/task-4-codebase-audit-roadmap.txt`
  Commit: Y | perf(commands): use LazyLock<HashMap> for O(1) command lookup

- [x] 5. Create rust-toolchain.toml to pin Rust version
  What to do / Must NOT do: Create `rust/rust-toolchain.toml` with `channel = "stable"` and explicit `components = ["clippy", "rustfmt"]`. This ensures CI clippy checks (Wave 2) are reproducible. Use the same Rust version as the Dockerfile base (1.75). Do NOT pin to nightly; do NOT add targets here (those are set in CI YAML).
  Parallelization: Wave 1 | Blocked by: none | Blocks: 7, 8, 9, 10 (CI clippy needs pinned toolchain)
  References: Dockerfile:1 (rust:1.75-slim), .github/workflows/parity.yml
  Acceptance criteria (agent-executable): `rust/rust-toolchain.toml` exists with `channel = "1.75"` and `components = ["clippy", "rustfmt"]`. `cargo +1.75 --version` succeeds. `cargo clippy --manifest-path rust/Cargo.toml -- -D warnings` runs with the pinned toolchain.
  QA scenarios: happy — CI clippy uses the same version as local dev. failure — verify toolchain mismatch is caught (wrong version causes build failure). Evidence `.omo/evidence/task-5-codebase-audit-roadmap.toml`
  Commit: Y | chore: add rust-toolchain.toml pinning Rust 1.75

- [x] 6. Update all Rust dependencies and consolidate rand versions
  What to do / Must NOT do: Run `cargo update` in rust/, wasm-lib/, and root. CRITICAL: The Cargo.lock currently has BOTH rand 0.8.6 AND rand 0.9.4 (pulled by proptest 1.11.0). After `cargo update`, bump `rand = "0.8"` to `rand = "0.9"` in rust/Cargo.toml and fix any API breakage in commands using rand (check shuf.rs for `SliceRandom` import path changes). If rand 0.9 API is incompatible, pin rand at 0.8 and add `[patch]` to suppress proptest's 0.9 pull. Do NOT blindly bump major versions without verifying shuf.rs still works.
  Parallelization: Wave 1 | Blocked by: none | Blocks: none (CI is independent of dep versions; audit will catch issues)
  References: rust/Cargo.toml:9 (rand = "0.8"), rust/Cargo.lock (rand 0.8.6 and 0.9.4), rust/src/commands/shuf.rs (uses rand), wasm-lib/Cargo.toml, vish/Cargo.toml
  Acceptance criteria (agent-executable): `cargo build --manifest-path rust/Cargo.toml` succeeds. `grep 'name = "rand"' rust/Cargo.lock` shows exactly ONE version entry (not two). `cargo test --manifest-path rust/Cargo.toml` — all tests pass including shuf tests. `cargo audit` reports zero vulnerabilities.
  QA scenarios: happy — single rand version in lockfile, all tests pass. failure — verify shuf still shuffles correctly. Evidence `.omo/evidence/task-7-codebase-audit-roadmap.txt`
  Commit: Y | chore(deps): update dependencies and consolidate rand to single version

### Wave 2: CI Quality Gates (P1)

- [x] 7. Add clippy linting to CI
  What to do / Must NOT do: Add a `clippy` job to `.github/workflows/parity.yml` that runs `cargo clippy --manifest-path rust/Cargo.toml -- -D warnings`. Set `continue-on-error: true` for the first month. Do NOT make it a required check yet — it may fail on existing warnings.
  Parallelization: Wave 2 | Blocked by: 5 | Blocks: none
  References: .github/workflows/parity.yml, rust/Cargo.toml
  Acceptance criteria (agent-executable): The CI workflow file contains a `clippy` job. After `cargo update` from Todo 5, `cargo clippy --manifest-path rust/Cargo.toml` produces a report (warnings are okay, just no hard errors breaking the build).
  QA scenarios: happy — push to branch triggers clippy in CI (visible in Actions tab). failure — clippy job exists but is misconfigured (wrong path). Evidence `.omo/evidence/task-7-codebase-audit-roadmap.yml`
  Commit: Y | ci: add clippy linting job (non-blocking)

- [x] 8. Add cargo-audit security scanning to CI
  What to do / Must NOT do: Add an `audit` job to `.github/workflows/parity.yml` that installs `cargo-audit` and runs `cargo audit --manifest-path rust/Cargo.toml`. Make it a required check (fail on vulnerabilities). Add `cargo audit --manifest-path wasm-lib/Cargo.toml` as well.
  Parallelization: Wave 2 | Blocked by: 5 | Blocks: none
  References: .github/workflows/parity.yml, rust/Cargo.toml, wasm-lib/Cargo.toml
  Acceptance criteria (agent-executable): CI workflow contains an `audit` job. Running `cargo audit` locally reports zero vulnerabilities after Todo 5 dependency update.
  QA scenarios: happy — push triggers audit, passes with zero vulns. failure — intentionally introduce a known-vulnerable dep version to confirm audit catches it. Evidence `.omo/evidence/task-7-codebase-audit-roadmap.yml`
  Commit: Y | ci: add cargo-audit security scanning

- [x] 9. Add benchmark job to CI
  What to do / Must NOT do: Add a `benchmark` job to `.github/workflows/parity.yml` that runs `make benchmark` (which calls `tooling/benchmark.py`). Make it informative (non-blocking). Capture output as a CI artifact or summary.
  Parallelization: Wave 2 | Blocked by: 5 | Blocks: none
  References: .github/workflows/parity.yml, Makefile:66 (benchmark target), tooling/benchmark.py
  Acceptance criteria (agent-executable): CI workflow contains a `benchmark` job. `make benchmark` runs and produces speedup ratios for Rust vs Python.
  QA scenarios: happy — benchmark runs in CI and produces output. failure — benchmark fails due to missing Python dependencies. Evidence `.omo/evidence/task-8-codebase-audit-roadmap.txt`
  Commit: Y | ci: add benchmark job (non-blocking)

- [x] 10. Add fuzz testing to CI
  What to do / Must NOT do: Add a `fuzz` job to `.github/workflows/parity.yml` that runs `cargo test --manifest-path rust/Cargo.toml --test fuzz`. Set a time limit (e.g., 5 minutes for proptest shrinking). Make it a required check.
  Parallelization: Wave 2 | Blocked by: 5 | Blocks: none
  References: .github/workflows/parity.yml, rust/tests/fuzz.rs (35 fuzz tests)
  Acceptance criteria (agent-executable): CI workflow contains a `fuzz` job. `cargo test --manifest-path rust/Cargo.toml --test fuzz` passes all 35 fuzz tests.
  QA scenarios: happy — fuzz tests run in CI and pass. failure — verify fuzz job fails when a command panics (add a temporary panic in a test to confirm). Evidence `.omo/evidence/task-9-codebase-audit-roadmap.yml`
  Commit: Y | ci: add fuzz testing job

### Wave 3: GVIBU Surface (P1)

- [x] 11. Design vish REPL protocol and command dispatch interface + plan test migration
  What to do / Must NOT do: Define the vish REPL behavior: prompt format (`gvibu> `), quit command (`exit` or Ctrl+D), help command, and how commands map. IMPORTANT: The existing `gvibu-linux/vish` (32-line bash script) and `gvibu-python/vish.py` (argparse MVP) will be REPLACED by the new vish implementations. Document what output format the new vish will produce vs. the old one. Plan the migration: (a) `tests/test_vish_cli.py` currently tests `./gvibu-linux/vish help` expecting "GVIBU minimal shell" — update these tests in Todo 13 to match the new output, (b) `tests/test_vish_py.py` and `tests/test_vish_integration_py.py` will be updated in Todo 18/19. The design doc must specify the exact help text, error message format, and exit codes so tests can be written against it. Do NOT implement a full shell (no pipes, no redirects, no variables, no scripting). Do NOT change the existing command run signatures.
  Parallelization: Wave 3 | Blocked by: none | Blocks: 12, 13, 14, 15
  References: rust/src/commands/mod.rs:70-133 (COMMANDS array), rust/src/main.rs:1-47 (existing dispatch), gvibu-linux/vish (existing bash script), gvibu-python/vish.py (argparse MVP), tests/test_vish_cli.py (tests bash script), tests/test_vish_py.py (tests Python MVP), tests/test_vish_integration_py.py (cross-lang tests)
  Acceptance criteria (agent-executable): A design doc exists at `docs/vish-design.md` specifying: prompt format, command dispatch flow, error handling, exit codes, help text strings, migration plan for existing test files, and what is explicitly NOT supported.
  QA scenarios: happy — design covers all 58 commands and defines exact output format for testability. failure — design leaves output format unspecified (ambiguity). Evidence `.omo/evidence/task-11-codebase-audit-roadmap.md`
  Commit: Y | docs: add vish shell design document with test migration plan

- [x] 12. Implement Rust vish binary — main loop and dispatch
  What to do / Must NOT do: Create `vish/src/main.rs` with: a REPL loop using `std::io::stdin().read_line()`, a prompt `gvibu> `, command lookup via `gvibu::commands::lookup()`, dispatch via `(cmd.run)(&mut io::stdout(), &args)`, and `exit` / `quit` built-in commands. Support subcommand mode: `gvibu <cmd> [args]` passes through. This REPLACES the existing `gvibu-linux/vish` bash script. The Cargo.toml at `vish/Cargo.toml` already exists — add src/main.rs to it. Do NOT implement pipes, redirects, job control, or scripting. Use the existing `gvibu` crate as a dependency.
  Parallelization: Wave 3 | Blocked by: 11 | Blocks: 19, 20
  References: rust/src/main.rs:1-47 (dispatch pattern), rust/src/commands/mod.rs, vish/Cargo.toml (stub), docs/vish-design.md (from Todo 11)
  Acceptance criteria (agent-executable): `cargo build --manifest-path vish/Cargo.toml` succeeds. Running `./target/debug/vish` starts a REPL. Typing `echo hello` prints `hello`. Typing `exit` exits with code 0. `echo "echo hello" | ./target/debug/vish` prints `hello` then exits (non-interactive mode).
  QA scenarios: happy — `vish` REPL: echo, true, false, pwd, ls all work. `echo "echo hello\nexit\n" | vish` works. `vish nonexistent` prints error. `vish` with empty input continues REPL. failure — verify `vish` exits gracefully on SIGTERM. Evidence `.omo/evidence/task-12-codebase-audit-roadmap.txt`
  Commit: Y | feat(vish): implement Rust vish REPL with command dispatch

- [x] 13. Add vish built-in commands (help, version, commands) + integration tests
  What to do / Must NOT do: Two parts: (A) Add `help` (prints usage), `version` (reads CARGO_PKG_VERSION from gvibu crate), and `commands` (lists all 58 command names) as built-in commands in the vish REPL. These are handled BEFORE command lookup, shadowing any same-named coreutils command (there are none). (B) Add integration tests: REPL with echo/true/false/pwd, exit command, unknown command error, empty input handling, built-in commands. Use subprocess or the run helpers from cli.rs. Do NOT add any other built-ins. Do NOT test pipes/redirects. Do NOT change any command signature.
  Parallelization: Wave 3 | Blocked by: 11, 12 | Blocks: none
  References: vish/src/main.rs (from Todo 12), rust/src/commands/mod.rs:70-129 (COMMANDS), rust/tests/cli.rs:1-64 (run helpers pattern), rust/Cargo.toml (gvibu crate version), tests/test_vish_cli.py
  Acceptance criteria (agent-executable): `cargo test` includes vish tests. At minimum: echo test, true/false exit codes, unknown command error, exit, help output, version output, commands listing. All pass. In REPL: `help` prints usage text, `version` prints version, `commands` lists 58 commands.
  QA scenarios: happy — all built-ins work; tests pass in CI. failure — intentionally break dispatch and verify test catches it. Evidence `.omo/evidence/task-13-codebase-audit-roadmap.txt`
  Commit: Y | feat(vish): add built-in commands and integration tests

- [x] 14. Implement Python vish — main script and dispatch
  What to do / Must NOT do: Create `gvibu-python/vish.py` (or update existing placeholder) with: a REPL loop using `input()`, prompt `gvibu> `, command dispatch via subprocess calling the Python reference implementation (`python-ref/gvibu_ref/main.py`), and `exit`/`quit` built-ins. Use the existing Python ref's main.py dispatch. Do NOT implement pipes, redirects, or scripting.
  Parallelization: Wave 3 | Blocked by: 11 | Blocks: 18, 19
  References: python-ref/gvibu_ref/main.py (existing dispatch), gvibu-python/vish.py (existing placeholder), gvibu-python/gvibu.py (existing placeholder)
  Acceptance criteria (agent-executable): `python3 gvibu-python/vish.py` starts a REPL. Typing `echo hello` prints `hello`. Typing `exit` exits. Piping input: `echo "echo hello\nexit" | python3 gvibu-python/vish.py` works non-interactively.
  QA scenarios: happy — Python vish REPL works for echo, true, false, pwd. Non-interactive mode with piped input. Unknown command prints error. failure — verify Python vish doesn't crash on special characters, empty input, very long input. Evidence `.omo/evidence/task-14-codebase-audit-roadmap.txt`
  Commit: Y | feat(vish): implement Python vish REPL with command dispatch

- [x] 15. Add Python vish built-in commands — help, list
  What to do / Must NOT do: Add `help` and `list` built-in commands to the Python vish REPL. Same behavior as Rust vish but Python-native. List available commands from the Python ref COMMANDS dict.
  Parallelization: Wave 3 | Blocked by: 14 | Blocks: none
  References: gvibu-python/vish.py (from Todo 14), python-ref/gvibu_ref/main.py (COMMANDS dict)
  Acceptance criteria (agent-executable): In Python vish REPL: `help` prints usage, `list` prints all 58 command names.
  QA scenarios: happy — built-ins work. failure — verify no shadowing of coreutils commands. Evidence `.omo/evidence/task-15-codebase-audit-roadmap.txt`
  Commit: Y | feat(vish): add help and list built-ins to Python vish

- [x] 16. Update Python Dockerfile for vish
  What to do / Must NOT do: Update `gvibu-python/Dockerfile` to copy the new vish.py and set CMD to `python3 /gvibu/vish.py` (or keep existing and verify it works). Ensure the Python reference is installed. Verify the HEALTHCHECK still works.
  Parallelization: Wave 3 | Blocked by: 14 | Blocks: none
  References: gvibu-python/Dockerfile (existing 10 lines), docker-compose.yaml (gvibu_python service)
  Acceptance criteria (agent-executable): `docker compose build gvibu_python` succeeds. `docker compose run --rm gvibu_python echo hello` prints expected output (if interactive, use `echo "echo hello\nexit" | docker compose run --rm -T gvibu_python`).
  QA scenarios: happy — Docker container runs vish and dispatches commands. failure — Docker build fails due to missing Python dependencies. Evidence `.omo/evidence/task-16-codebase-audit-roadmap.txt`
  Commit: Y | fix(docker): update Python Dockerfile for vish shell

- [x] 17. Update Rust Dockerfile for vish
  What to do / Must NOT do: Update `gvibu-rust/Dockerfile` to build and copy the vish binary. Set CMD to `vish`. Verify the HEALTHCHECK still works if present.
  Parallelization: Wave 3 | Blocked by: 11 | Blocks: none
  References: gvibu-rust/Dockerfile (existing 11 lines), docker-compose.yaml (gvibu_rust service)
  Acceptance criteria (agent-executable): `docker compose build gvibu_rust` succeeds. `echo "echo hello\nexit" | docker compose run --rm -T gvibu_rust` prints expected output.
  QA scenarios: happy — Docker container runs vish and dispatches commands. failure — Docker build fails due to vish not being found. Evidence `.omo/evidence/task-17-codebase-audit-roadmap.txt`
  Commit: Y | fix(docker): update Rust Dockerfile for vish binary

### Wave 4: Platform & Docs (P2)

- [x] 18. Add Python vish CLI tests
  What to do / Must NOT do: Add tests to `tests/test_vish_py.py` (existing) or create new. Test REPL with echo/true/false, exit, unknown command, built-ins. Use pytest with subprocess.
  Parallelization: Wave 4 | Blocked by: 14 | Blocks: none
  References: tests/test_vish_py.py, gvibu-python/vish.py
  Acceptance criteria (agent-executable): `pytest tests/test_vish_py.py -v` passes all tests. At least: echo test, true exit code, false exit code, unknown command, help output, exit behavior.
  QA scenarios: happy — all Python vish tests pass. failure — verify test catches a broken dispatch. Evidence `.omo/evidence/task-18-codebase-audit-roadmap.txt`
  Commit: Y | test(vish): add Python vish CLI tests

- [x] 19. Add vish integration test (cross-language parity)
  What to do / Must NOT do: Create or update `tests/test_vish_integration_py.py` (existing file) to verify Rust vish and Python vish produce identical output for a subset of commands (echo, true, false, pwd, basename, whoami). Use subprocess for both.
  Parallelization: Wave 4 | Blocked by: 14, 11 | Blocks: none
  References: tests/test_vish_integration_py.py, vish/src/main.rs, gvibu-python/vish.py
  Acceptance criteria (agent-executable): `pytest tests/test_vish_integration_py.py -v` passes. Rust and Python vish produce identical output for tested commands.
  QA scenarios: happy — parity between implementations. failure — verify test catches divergence (change one output format to confirm). Evidence `.omo/evidence/task-19-codebase-audit-roadmap.txt`
  Commit: Y | test(vish): add cross-language vish parity tests

- [x] 20. Add QEMU run script for Python GVIBU image
  What to do / Must NOT do: Update `gvibu-linux/qemu_run_py.sh` (existing stub, currently only `echo`s the command) to actually build initramfs including Python ref and vish.py, then invoke QEMU via `gvibu-linux/run_qemu.sh`. The script must actually execute QEMU (not just print instructions). If kernel is not found at the hardcoded path, print a clear error with instructions. Follow the pattern from `gvibu-linux/build_initramfs.sh` and `gvibu-linux/run_qemu.sh`. Do NOT hardcode a new kernel path — reuse the existing one or document the prerequisite.
  Parallelization: Wave 4 | Blocked by: 14 | Blocks: none
  References: gvibu-linux/build_initramfs.sh:1-129 (initramfs builder), gvibu-linux/run_qemu.sh:1-36 (kernel path: /boot/vmlinuz-6.12.90+deb13.1-cloud-amd64), gvibu-linux/qemu_run_py.sh (existing 11-line stub)
  Acceptance criteria (agent-executable): `bash gvibu-linux/qemu_run_py.sh 2>&1 | head -5` shows QEMU boot messages or a clear prerequisite-error. If kernel+QEMU are present: script boots to a shell where `python3 /gvibu/vish.py` starts the REPL. If absent: script exits with clear message listing requirements.
  QA scenarios: happy — script either boots QEMU or gives clear instructions. failure — verify script doesn't just echo and exit 0 without doing anything. Evidence `.omo/evidence/task-20-codebase-audit-roadmap.txt`
  Commit: Y | feat(qemu): implement Python GVIBU QEMU run script

- [x] 21. Add QEMU run script for Rust GVIBU image
  What to do / Must NOT do: Update `gvibu-linux/qemu_run_rs.sh` (existing stub, currently only `echo`s the command) to actually build initramfs including vish Rust binary and invoke QEMU. Same requirements as Todo 20: must execute QEMU or fail with clear prerequisite error. Do NOT just echo the command. Follow existing patterns.
  Parallelization: Wave 4 | Blocked by: 11 | Blocks: none
  References: gvibu-linux/build_initramfs.sh, gvibu-linux/run_qemu.sh, gvibu-linux/qemu_run_rs.sh (existing 11-line stub)
  Acceptance criteria (agent-executable): `bash gvibu-linux/qemu_run_rs.sh 2>&1 | head -5` shows QEMU boot messages or a clear prerequisite-error. If kernel+QEMU are present: script boots to a shell where `vish` starts the REPL.
  QA scenarios: happy — script either boots QEMU or gives clear instructions. failure — verify script doesn't just echo and exit 0. Evidence `.omo/evidence/task-21-codebase-audit-roadmap.txt`
  Commit: Y | feat(qemu): implement Rust GVIBU QEMU run script

- [x] 22. Implement WASM stdin support via dispatch-level injection
  What to do / Must NOT do: Modify `wasm-lib/src/lib.rs` to support stdin. The existing `run_command` already accepts `_stdin: Option<String>` — implement the dispatch path to actually use it. Mechanism: add a `static STDIN_BUFFER: OnceLock<String>` in the `gvibu` crate (NOT `static mut` — use `std::sync::OnceLock`). Expose a `pub fn get_wasm_stdin() -> Option<&'static str>` helper. Commands that read stdin (wc, cat, grep, sort, uniq, head, tail, tr, cut, fold) will need a ONE-LINE change: check `get_wasm_stdin()` before falling through to `io::stdin()`. The `run()` signature `(w: &mut dyn Write, args: &[String]) -> i32` MUST NOT change — only the stdin-reading logic inside the command body is touched. Do NOT implement filesystem access in WASM.
  Parallelization: Wave 4 | Blocked by: none | Blocks: none
  References: wasm-lib/src/lib.rs:1-71 (current WasmWriter, run_command with _stdin param), rust/src/commands/wc.rs:122 (io::stdin().lock()), wc.rs, cat.rs, sort.rs, uniq.rs, head.rs, tail.rs, tr.rs, cut.rs, fold.rs, grep.rs (all stdin-reading commands)
  Acceptance criteria (agent-executable): `wasm-pack build wasm-lib --target web` succeeds. All existing tests pass. New test: `run_command("wc", ["-l"], Some("line1\nline2\n"))` returns `"2"`. `run_command("echo", ["hello"], None)` still works.
  QA scenarios: happy — stdin-using commands work in WASM with provided stdin. failure — commands requiring filesystem still fail gracefully (return error). Evidence `.omo/evidence/task-22-codebase-audit-roadmap.txt`
  Commit: Y | feat(wasm): add dispatch-level stdin support for WASM commands

- [x] 23. Update README with Linux userspace testing instructions
  What to do / Must NOT do: Add a section to README.md covering: how to test gvibu in QEMU (Rust and Python), prerequisites (kernel, QEMU), and troubleshooting. Reference the gvibu-linux/ directory. Do NOT duplicate the full README content.
  Parallelization: Wave 4 | Blocked by: 20, 21 | Blocks: none
  References: README.md:1-116, gvibu-linux/README.md
  Acceptance criteria (agent-executable): README.md contains a "Linux Userspace Testing" section with QEMU instructions for both Rust and Python variants. Links to gvibu-linux/ are valid.
  QA scenarios: happy — instructions are followable by a new user. failure — verify no broken links. Evidence `.omo/evidence/task-23-codebase-audit-roadmap.md`
  Commit: Y | docs: add Linux userspace testing instructions to README

- [x] 24. Clean up Ralph tasks — mark completed items
  What to do / Must NOT do: Update `.ralph/ralph-tasks.md` to mark tasks completed by this plan. Tasks for vish shell, QEMU scripts, Dockerfiles, docker-compose, CLI tests, README update — all become `[x]` when their corresponding todos are done. Do NOT delete any tasks.
  Parallelization: Wave 4 | Blocked by: 11, 14, 16, 17, 20, 21, 23 | Blocks: none
  References: .ralph/ralph-tasks.md:1-17
  Acceptance criteria (agent-executable): All 13 tasks in ralph-tasks.md are either `[x]` (done) or `[/]` (partially done with note). No tasks remain `[ ]`.
  QA scenarios: happy — ralph tasks reflect actual implementation status. failure — verify no task was incorrectly marked done. Evidence `.omo/evidence/task-24-codebase-audit-roadmap.md`
  Commit: Y | chore: update Ralph task status for completed work

- [x] 25. Fix empty opencode-container/docker-compose.yaml
  What to do / Must NOT do: Either populate `opencode-container/docker-compose.yaml` with a valid OpenCode container service definition, or remove the empty file if it serves no purpose. If populating: define a service using the gvibu Docker image with appropriate volumes and command.
  Parallelization: Wave 4 | Blocked by: none | Blocks: none
  References: opencode-container/docker-compose.yaml (0 bytes), docker-compose.yaml (root, for pattern)
  Acceptance criteria (agent-executable): `opencode-container/docker-compose.yaml` is either a valid docker-compose file with a service definition, or the file no longer exists (if removed).
  QA scenarios: happy — file is valid YAML with a service. failure — `docker compose -f opencode-container/docker-compose.yaml config` fails (if populated). Evidence `.omo/evidence/task-25-codebase-audit-roadmap.yml`
  Commit: Y | fix: populate or remove empty opencode-container/docker-compose.yaml

### Wave 5: Final Verification (P3)

- [x] 26. Run full parity test suite (Rust vs Python)
  What to do / Must NOT do: Execute `make compare` which runs `tooling/compare_impls.py` testing all 58 commands against shared test cases. Verify ALL pass. If any fail, fix before proceeding.
  Parallelization: Wave 5 | Blocked by: 1, 2, 3, 4, 5 | Blocks: none
  References: Makefile:66 (compare target), tooling/compare_impls.py, shared-tests/cases/*.json (58 files)
  Acceptance criteria (agent-executable): `make compare` exits with code 0. All 58 commands pass parity with their Python reference implementation.
  QA scenarios: happy — 100% parity pass rate. failure — identify and fix any regressions introduced by unwrap/dep changes. Evidence `.omo/evidence/task-26-codebase-audit-roadmap.txt`
  Commit: N (verification only)

- [x] 27. Run full test suite (all languages, all targets)
  What to do / Must NOT do: Execute: `make rust-test`, `make python-test`, `cargo test --manifest-path rust/Cargo.toml --test fuzz`, `pytest tests/test_vish_cli.py tests/test_vish_py.py tests/test_vish_integration_py.py -v`. All must pass.
  Parallelization: Wave 5 | Blocked by: 1-25 | Blocks: none
  References: Makefile targets, rust/tests/cli.rs (255 tests), rust/tests/fuzz.rs (35 tests), tests/ directory (58 Python tests)
  Acceptance criteria (agent-executable): All test suites pass with zero failures. Total test count meets or exceeds baseline: 255 CLI + 35 fuzz + 58 Python + new vish tests.
  QA scenarios: happy — all tests green. failure — identify any broken tests and their root cause in this plan's changes. Evidence `.omo/evidence/task-27-codebase-audit-roadmap.txt`
  Commit: N (verification only)

- [x] 28. Verify WASM build and browser demo
  What to do / Must NOT do: Execute `make wasm-browser-build` and `make wasm-test`. Verify the browser demo at `wasm-lib/index.html` loads and dispatches commands (especially stdin-using ones from Todo 22). Run wasmtime smoke tests.
  Parallelization: Wave 5 | Blocked by: 22 | Blocks: none
  References: Makefile:26-48 (wasm targets), wasm-lib/index.html, wasm-lib/src/lib.rs
  Acceptance criteria (agent-executable): `make wasm-browser-build` succeeds. `make wasm-test` passes (true, echo, false all work). Browser demo loads without JS errors.
  QA scenarios: happy — WASM build and tests pass. failure — WASM build fails due to dependency changes. Evidence `.omo/evidence/task-28-codebase-audit-roadmap.txt`
  Commit: N (verification only)

- [x] 29. Verify Docker builds (all variants)
  What to do / Must NOT do: Execute `docker compose build` for gvibu_python and gvibu_rust services. Verify both containers run and vish works in each. Execute `make docker-build` for the root Dockerfile.
  Parallelization: Wave 5 | Blocked by: 11, 14, 16, 17 | Blocks: none
  References: docker-compose.yaml, gvibu-python/Dockerfile, gvibu-rust/Dockerfile, Dockerfile (root)
  Acceptance criteria (agent-executable): All three Docker images build and run successfully. vish REPL works in gvibu_python and gvibu_rust containers. Root image runs `gvibu true` successfully.
  QA scenarios: happy — all Docker builds green. failure — identify build failures and fix Dockerfiles. Evidence `.omo/evidence/task-29-codebase-audit-roadmap.txt`
  Commit: N (verification only)

- [x] 30. Verify CI pipeline — all jobs pass
  What to do / Must NOT do: Push all changes to a branch and verify `.github/workflows/parity.yml` passes all jobs: test, wasm, docker, clippy (not required but runs), audit, benchmark, fuzz. The release workflow should be verified by inspection (no tag push).
  Parallelization: Wave 5 | Blocked by: 1-25 | Blocks: none
  References: .github/workflows/parity.yml, .github/workflows/release.yml
  Acceptance criteria (agent-executable): All CI jobs on the parity workflow pass. New jobs (clippy, audit, benchmark, fuzz) are visible and running. Audit reports zero vulnerabilities.
  QA scenarios: happy — CI green across all jobs. failure — identify failing CI jobs and their root cause. Evidence `.omo/evidence/task-30-codebase-audit-roadmap.txt`
  Commit: N (verification only)

## Final verification wave
> Runs in parallel after ALL todos. ALL must APPROVE. Surface results and wait for the user's explicit okay before declaring complete.
- [x] F1. Plan compliance audit — verify all 30 todos completed, all commits match plan, no scope creep
- [x] F2. Code quality review — clippy clean, no new unwrap/panic in production, SAFETY comments present
- [x] F3. Real manual QA — run the full command matrix: 58 commands × 3 invocations each = 174 executions, all pass
- [x] F4. Scope fidelity — verify nothing in MUST NOT was added; verify all MUST HAVE items are complete

## Commit strategy
- One commit per todo (where marked Y) — 24 implementation commits + verification (no-commit) items
- Commit format: `type(scope): summary` following existing conventions
- Wave 1 commits: fix + perf + docs
- Wave 2 commits: ci
- Wave 3 commits: feat + test + fix
- Wave 4 commits: feat + test + docs + chore
- Wave 5: no commits (verification only)

## Success criteria
1. `cargo audit` reports zero vulnerabilities (P0)
2. All 8 production unwrap/expect sites replaced with error handling (P0)
3. CI runs clippy, audit, benchmark, and fuzz on every push (P1)
4. Rust vish REPL dispatches all 58 commands (P1)
5. Python vish REPL dispatches all 58 commands (P1)
6. `make compare` passes 100% parity (58/58 commands) (P3)
7. All Docker images build and run (P3)
8. `make wasm-browser-build && make wasm-test` passes (P3)
9. All 31 todos marked complete with evidence files
