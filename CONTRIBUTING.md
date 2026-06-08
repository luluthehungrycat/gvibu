# Contributing to gvibu

## Adding a New Command

Follow this 8-step process (also captured as the `gvibu-new-command` skill):

### 1. Rust Module
Create `rust/src/commands/cmdname.rs`:
```rust
use std::io::Write;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    // Implementation using write!/writeln! (not print!/println!)
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }
}
```

### 2. Python Module
Create `python-ref/gvibu_ref/commands/cmdname.py`:
```python
def run(args: list[str]) -> int:
    return 0
```

### 3. Spec
Create `specs/commands/cmdname.md` documenting usage, flags, exit codes.

### 4. Shared Tests
Create `shared-tests/cases/cmdname.json` with test cases:
```json
[
  {"args": [], "expected_exit_code": 0, "expected_stdout": ""}
]
```

### 5. Python Tests
Create `tests/test_cmdname.py` with pytest test functions.

### 6. Register (8 files)
- `rust/src/commands/mod.rs` — add `pub mod cmdname;` + `Command { names: &["cmdname"], run: cmdname::run }`
- `python-ref/gvibu_ref/main.py` — add to COMMANDS dict, both try/except branches, assignment
- `tooling/compare_impls.py` — add to command list
- `Makefile` install target — add symlink
- `Dockerfile` + `rust/Dockerfile` — add symlink
- `gvibu-linux/build_initramfs.sh` — add to symlink loop

### 7. Integration Tests
Add tests to `rust/tests/cli.rs` using the `run()`, `run_with_stdin()`, or `run_with_timeout()` helpers.

### 8. WASM
If the command has new native dependencies (e.g., `libc` functions), add them to `wasm-lib/Cargo.toml`.

## Conventions

- **Rust:** `pub fn run(w: &mut dyn Write, args: &[String]) -> i32`
- **Python:** `def run(args: list[str]) -> int`
- **Errors:** Print to stderr (`eprintln!` / `print(..., file=sys.stderr)`)
- **Exit codes:** 0 success, 1 generic error, 2 usage error
- **Flags:** Parse iteratively from args start; allow combined short flags (`-lwcm`)

## Testing

```bash
# Rust tests
cargo test

# Python tests
pytest tests/ -v

# Parity tests (must pass both)
make compare

# Fuzzing
cargo test --test fuzz

# Full benchmark
make benchmark
```

All PRs must pass CI (test, docker, and wasm jobs) before merging.
