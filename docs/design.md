# Design Decisions

## Why Rust + Python?

### Rust (Production)
- **Performance**: 28–113× faster than the Python reference
- **Safety**: Memory-safe by default, no GC pauses
- **Deployment**: Single static binary, WASM target, Docker-friendly

### Python (Reference)
- **Readability**: Clear, explicit logic for understanding behavior
- **Parity**: Shared test cases guarantee identical semantics
- **Development speed**: Faster iteration on edge cases before porting to Rust

## Testing Philosophy

### Three-Layer Testing

1. **Unit tests** (inline `#[cfg(test)]`) — Test individual functions in isolation
2. **Integration tests** (`cli.rs`) — Run the compiled binary as a subprocess, testing end-to-end behavior
3. **Property-based tests** (`fuzz.rs`) — Random argument generation to catch panics and edge cases

### Shared Test Cases

All command semantics are defined by `shared-tests/cases/*.json`, which both Rust and Python implementations must pass. This ensures:

- Identical behavior across implementations
- A single source of truth for expected behavior
- Easy addition of new test cases

### Why Subprocess Integration Tests?

Integration tests run the actual compiled binary via `std::process::Command` rather than calling the `run()` function directly. This:

- Tests the full dispatch pipeline (arg parsing → resolution → execution)
- Catches linker issues, missing symbols, and panic unwinding bugs
- Provides realistic isolation (separate process, real stdio)
- Works with timeouts for infinite-loop commands like `yes`

## Writer-Output Pattern

Commands were refactored from direct `println!`/`print!` to `write!`/`writeln!` on a `&mut dyn Write` parameter for WASM compatibility. This was a mechanical change with no behavioral difference for CLI usage.

### Refactoring Strategy
1. Changed the `Command` struct signature
2. Updated `main.rs` to pass `&mut io::stdout()`
3. Updated each command module individually
4. Confirmed all tests pass with `&mut io::sink()` in unit tests

## Flag Parsing Convention

Flags are parsed **iteratively from the start of args**:

```rust
let mut i = 0;
while i < args.len() {
    if args[i] == "-n" { flag_n = true; }
    else if args[i] == "-r" { flag_r = true; }
    else if !args[i].starts_with('-') || args[i] == "-" { break; }
    i += 1;
}
```

Short flags can often be combined (`-lwcm` for `wc`). Combined flags are parsed by iterating over individual characters.

## Error Handling

- **Exit code 0**: Success
- **Exit code 1**: Generic error (nonexistent file, permission denied)
- **Exit code 2**: Usage error (missing operand, invalid option)
- **Errors go to stderr**: `eprintln!` in Rust, `print(..., file=sys.stderr)` in Python

## Why Not libc Directly?

Most commands use only `std::fs` and `std::io`. Only `id` and `kill` require `libc` for system calls (`getuid`, `getgid`, `kill`, etc.). These dependencies are gated:

```toml
[dependencies]
libc = "0.2"    # Only used by id.rs and kill.rs
```

## Implementation Priorities

Commands were prioritized by:
1. **Simplicity** → quick wins (true, false, echo, pwd)
2. **Utility** → practical shell scripting (cat, wc, head, sort)
3. **Coverage** → rounding out the coreutils set (id, kill, chmod)
4. **Completeness** → flag support for production readiness (wc -m, seq -w, sort -k)

This prioritization ensures rapid early progress with increasing sophistication over time.
