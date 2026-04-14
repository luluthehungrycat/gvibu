# Command Status

## MVP Commands

| Command | Spec | Python (gvibu-ref) | Rust (gvibu) | Notes |
|---------|------|-------------------|--------------|-------|
| true    | [specs/commands/true.md](specs/commands/true.md) | - | - | Not started |
| false   | [specs/commands/false.md](specs/commands/false.md) | - | - | Not started |
| echo    | [specs/commands/echo.md](specs/commands/echo.md) | Python implementation | Rust implementation in progress | Not started |
| basename| [specs/commands/basename.md](specs/commands/basename.md) | Python implementation | Rust implementation in progress | Not started |
| pwd     | [specs/commands/pwd.md](specs/commands/pwd.md) | Python implementation | Rust implementation being developed | Not started |
| dirname | - | - | - | Scaffolderd |
| cat     | - | - | - | Scaffolderd |
| head    | - | - | - | Scaffolderd |
| wc      | - | - | - | Scaffolderd |

## Status Legend

- **Not started**: No implementation work begun
- **Spec pending**: Specification being written
- **Spec complete**: Specification finished, implementation pending
- **Python implementation**: Python reference being developed
- **Python tested**: Python implementation passing tests
- **Rust implementation**: Rust implementation being developed
- **Rust tested**: Rust implementation passing tests
- **Complete**: Both implementations passing parity tests
- **Scaffolderd**: Stub created but not built out

## Current Focus

Currently implementing: Phase 1 - Skeleton and first commands (true, false, echo, pwd)

## Generating This Table

Run: `python3 tooling/generate_command_status.py`
