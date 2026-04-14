# Migration Plan: Python to Rust

## Overview

This document describes the process for migrating commands from the Python reference implementation to Rust.

## Migration Criteria

A command is ready for Rust migration when it meets all of the following:

1. **Spec Finalized**: Human-readable spec in `specs/commands/` is complete
2. **Tests Complete**: Machine-readable tests in `shared-tests/cases/` cover all behaviors
3. **Python Implementation Stable**: Reference implementation passes all tests
4. **Parity Verified**: Both implementations produce identical output for all test cases

## Migration Process

### Step 1: Command Analysis
- Review Python reference implementation
- Identify all edge cases and behaviors
- Document any differences from POSIX/GNU

### Step 2: Spec Creation
- Write human-readable spec in `specs/commands/<command>.md`
- Create machine-readable tests in `shared-tests/cases/<command>.json`

### Step 3: Python Implementation
- Implement command in `python-ref/gvibu_ref/commands/<command>.py`
- Run tests, fix bugs until passing
- Verify against spec

### Step 4: Rust Implementation
- Implement command in `rust/src/commands/<command>.rs`
- Run tests, fix bugs until passing
- Verify against spec

### Step 5: Parity Verification
- Run `tooling/compare_impls.py` for the command
- Fix any discrepancies
- Ensure 100% parity before marking complete

## Parity Verification

The `compare_impls.py` tool:
1. Loads shared test cases from `shared-tests/cases/`
2. Runs each test against both implementations
3. Compares stdout, stderr, and exit codes
4. Reports any mismatches

## Command Readiness Checklist

- [ ] Spec written and reviewed
- [ ] Test cases written and passing
- [ ] Python implementation complete and tested
- [ ] Rust implementation complete and tested
- [ ] Parity verified (100% match)

## Post-Migration

After successful migration:
1. Update `docs/command-status.md` to mark Rust as "complete"
2. Keep Python reference for comparison/debugging
3. Both implementations continue to receive bug fixes

## Rollback

If Rust implementation has critical issues:
1. Document the problem in issue tracker
2. Continue using Python reference
3. Fix Rust implementation before further testing

## Non-Goals

- No automatic code generation between implementations
- No removal of Python reference (remains as fallback)
- No rush to migrate - stability over speed
