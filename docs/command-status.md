# Command Status

## MVP Commands

| Command  | Spec | Python (gvibu-ref) | Rust (gvibu) | Notes |
|----------|------|-------------------|--------------|-------|
| true     | Done | Done | Done | Complete. Returns 0 always. |
| false    | Done | Done | Done | Complete. Returns 1 always. |
| echo     | Done | Done | Done | Complete. Supports `-n` flag (no trailing newline). |
| basename | Done | Done | Done | Complete. Extracts filename, optional suffix removal. |
| pwd      | Done | Done | Done | Complete. Prints working directory path. |
| dirname  | -    | -    | -    | Scaffolded — not yet implemented |
| cat      | -    | -    | -    | Scaffolded — not yet implemented |
| head     | -    | -    | -    | Scaffolded — not yet implemented |
| wc       | -    | -    | -    | Scaffolded — not yet implemented |

## Status Legend

- **Done**: Implementation complete and parity-verified
- **In Progress**: Being actively developed
- **Scaffolded**: Stub created but not built out
- **Not started**: No implementation work begun

## Current Focus

All 5 MVP commands are fully implemented and parity-verified. Next: implement dirname, cat, head, and wc.

## Generating This Table

Run: `python3 tooling/generate_command_status.py`
