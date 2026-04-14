# gvibu Architecture

## Overview

gvibu is an experimental Unix-style multicall utility suite with partial coreutils compatibility. It follows the BusyBox model where a single binary provides multiple commands.

## Design Principles

1. **Multicall Binary**: Single binary (`gvibu`) dispatches to different commands based on invocation
2. **Dual Implementation**: Python reference (`gvibu-ref`) for rapid prototyping, Rust (`gvibu`) for production
3. **Shared Specifications**: Human-readable specs and machine-readable tests are shared between implementations
4. **Deterministic Behavior**: Commands produce consistent, testable output

## Dispatch Modes

### Subcommand Mode
```
gvibu echo hello
gvibu pwd
```
The command name is the first argument after the program name.

### Symlink Dispatch Mode
```
./echo hello
./pwd
```
The binary is invoked via a symlink named after the command. The program name (`argv[0]`) contains the command name.

## Directory Structure

```
gvibu-ai-lab/
├── docs/                  # Architecture, plans, policies
├── specs/                # Human-readable command specifications
│   └── commands/         # Individual command specs
├── shared-tests/         # Machine-readable test cases
│   └── cases/           # JSON test cases per command
├── python-ref/           # Python reference implementation
│   └── gvibu_ref/       # Main package
│       └── commands/    # Individual command modules
├── rust/                 # Rust implementation
│   └── src/
│       └── commands/    # Individual command modules
├── tooling/             # Build and comparison scripts
└── Makefile            # Root orchestrator
```

## Exit Codes

- `0`: Success
- `1`: Runtime error
- `2`: Usage error (invalid arguments, missing operands)

## Output Conventions

- **stdout**: Normal command output
- **stderr**: Error messages and diagnostics

## Command Interface

Commands are implemented as modules with a standardized interface:

### Python
```python
def run(args: list[str]) -> int:
    """Execute command with args, return exit code."""
    return 0
```

### Rust
```rust
fn run(args: &[std::ffi::OsString]) -> i32 {
    // Execute command with args, return exit code
    0
}
```

## Testing Strategy

1. **Shared Test Cases**: JSON files in `shared-tests/cases/` define inputs and expected outputs
2. **Implementation Tests**: Each implementation runs the same test cases
3. **Parity Verification**: `tooling/compare_impls.py` runs identical tests against both implementations
