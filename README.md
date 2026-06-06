# gvibu-ai-lab

gvibu is an experimental Unix-style multicall utility suite with partial coreutils compatibility.
It is built as a learning and systems-design project, with a Python reference implementation and a Rust implementation sharing behavior specs and tests.
The goal is not full GNU replacement, but clean CLI design, deterministic behavior, and incremental migration from reference prototype to production-minded binary.

## Architecture

- **Multicall binary**: Single binary dispatches to commands via subcommand or symlink
- **Dual implementation**: Python reference (`python-ref/gvibu_ref`) for rapid prototyping, Rust (`rust/`) for production
- **Shared specs**: Human-readable specs in `specs/commands/` and machine-readable tests in `shared-tests/cases/`
- **Parity verification**: `tooling/compare_impls.py` runs identical test cases against both implementations

## Implemented Commands (MVP)

| Command  | Spec | Python | Rust | Rust Tests | Notes |
|----------|------|--------|------|------------|-------|
| true     | ✓    | ✓      | ✓    | ✓          | Returns 0 |
| false    | ✓    | ✓      | ✓    | ✓          | Returns 1 |
| echo     | ✓    | ✓      | ✓    | ✓          | Supports `-n` flag |
| pwd      | ✓    | ✓      | ✓    | ✓          | Prints working directory |
| basename | ✓    | ✓      | ✓    | ✓          | Path extraction with suffix removal |
| dirname  | ✓    | ✓      | ✓    | ✓          | Directory portion of a file path |
| cat      | ✓    | ✓      | ✓    | ✓          | Concatenate files, stdin support |
| wc       | ✓    | ✓      | ✓    | ✓          | Line/word/byte counts, combined flags |
| head     | ✓    | ✓      | ✓    | ✓          | First N lines, stdin support, multi-file |

**Total: 9 commands — 9 specs, 9 Python impls, 9 Rust impls, 35 Rust integration tests, 60 Python tests, 50+ parity tests**

## Quick Start

```bash
# Run Python reference
python3 python-ref/gvibu_ref/main.py <command> [args...]

# Build and run Rust implementation
cd rust && cargo build
./target/debug/gvibu <command> [args...]

# Run parity tests
python3 tooling/compare_impls.py

# Run Python unit tests
pytest tests/

# Run Rust tests
cd rust && cargo test
```

## Project Structure

```
gvibu-ai-lab/
├── specs/commands/       # Human-readable command specifications
├── shared-tests/cases/   # JSON test cases shared across implementations
├── python-ref/           # Python reference implementation
│   └── gvibu_ref/
│       └── commands/     # Individual command modules
├── rust/                 # Rust production implementation
│   ├── src/commands/     # Individual command modules
│   └── tests/            # Rust integration tests
├── tooling/              # Build, comparison, and status scripts
├── tests/                # Python unit tests
└── gvibu-linux/          # Linux initramfs/QEMU end-to-end scaffolding
```

## Design Principles

1. **Deterministic behavior**: Same input always produces same output
2. **Clean exit codes**: 0 = success, 1 = runtime error, 2 = usage error
3. **No feature creep**: Implement only what's specified
4. **Parity first**: Both implementations must agree on all test cases
