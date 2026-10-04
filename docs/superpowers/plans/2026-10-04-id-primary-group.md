# GVIBU id Primary Group Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Rust and Python `id -G` always include the selected primary group once.

**Architecture:** Add pure group-list composition helpers in each implementation. `-G` uses effective GID, or real GID with `-r`; supplementary order is retained and an already-present primary ID is not duplicated. Existing name conversion handles `-n`.

**Tech Stack:** Rust, Python 3, Cargo, pytest.

**Spec:** `docs/superpowers/specs/2026-10-04-id-primary-group-design.md`

## Global Constraints

- Keep default `id`, `-u`, and `-g` behavior unchanged.
- Add no dependencies and leave lockfiles unchanged.
- Run Cargo with `CARGO_TARGET_DIR=/workspace/.onboarding/targets/gvibu` to avoid tracked stale target artifacts.
- Do not assume the test machine has supplementary groups configured.

## Review Focus

- Empty supplementary groups still emit the effective group for `-G`.
- `-rG` uses the real GID and does not accidentally use effective GID.
- An existing primary ID is not duplicated.
- `-nG` resolves all emitted IDs to group names.
- Existing unrelated identity flags and default output remain stable.

---

### Task 1: Rust and Python `id -G` semantics

**Files:**
- Modify: `rust/src/commands/id.rs`
- Modify: `rust/tests/cli.rs`
- Modify: `python-ref/gvibu_ref/commands/id.py`
- Modify: `tests/test_id.py`

**Interfaces:**
- Rust produces `include_primary_group(groups: Vec<u32>, primary_gid: u32) -> Vec<u32>`.
- Python produces `_include_primary_group(groups: list[int], primary_gid: int) -> list[int]`.
- Both `-G` paths pass `getgroups()` plus effective GID, or real GID with `-r`, through the helper before formatting.

- [x] **Step 1: Add Rust helper and CLI regression tests.** Assert empty input yields the primary ID, preexisting primary is not duplicated, order is preserved, and `-G` output contains the selected GID.
- [x] **Step 2: Run the Rust tests to verify RED.** Run the focused `id` CLI test and module tests with the external target directory.

Expected: the new helper tests fail to compile or the CLI assertion fails because current `-G` returns no primary ID.

- [x] **Step 3: Implement the Rust helper and wire `-G` to it.** Keep `-n` formatting after group composition and select `get_gid()` only for `-rG`; otherwise use `get_egid()`.
- [x] **Step 4: Run the focused Rust tests to verify GREEN.**

Expected: all new `id` tests and the prior `id_supp_groups` test pass.

- [x] **Step 5: Add Python helper unit cases and CLI assertions first.** Cover empty input, duplicate suppression, order, numeric output, `-nG`, and `-rG` selection.
- [x] **Step 6: Run `PYTHONPATH=python-ref python3 -m pytest -q tests/test_id.py` to verify RED.**

Expected: the new empty-group case fails because `_include_primary_group` is not implemented.

- [x] **Step 7: Implement `_include_primary_group` and wire Python `-G` through it.** Choose the real GID only with `-r`, otherwise effective GID; retain existing name formatting.
- [x] **Step 8: Rerun focused Python tests to verify GREEN.**

Expected: all focused `id` tests pass.



- [x] **Step 9: Run complete required validation.** Run `CARGO_TARGET_DIR=/workspace/.onboarding/targets/gvibu cargo test --locked --manifest-path rust/Cargo.toml`, `PYTHONPATH=python-ref python3 -m pytest -q tests/`, and the existing Python/Rust parity script with its Rust binary pointed at `/workspace/.onboarding/targets/gvibu/debug/gvibu`.

Expected: all suites and all 266 parity cases pass.

- [x] **Step 10: Try optional WASM builds after required tests pass.** Install only missing Rust targets; run the locked `wasm32-wasip1` release build and `wasm-pack build wasm-lib --target web --out-dir pkg` if wasm-pack is available/installable. Run existing wasmtime smoke cases if the runtime is present.

Expected: builds pass, or the optional runtime/tool blocker is reported without changing the required-task result.

- [ ] **Step 11: Commit the GVIBU implementation and tests.**
