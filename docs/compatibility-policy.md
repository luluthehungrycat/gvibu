# Compatibility Policy

## Scope

gvibu provides partial coreutils-compatible commands. This document clarifies what compatibility means and what deviations are intentional.

## Standards

We aim for POSIX compliance where practical, with selective GNU compatibility. Full POSIX/POSIX compliance is NOT a goal.

## Intentional Incompatibilities

These behaviors differ from GNU/coreutils by design:

1. **Limited Options**: Only specified options are implemented
2. **No Long Options**: Single-character flags only (e.g., `-n`, not `--newline`)
3. **Strict Argument Handling**: No argument permuting or option parsing tricks
4. **No Extended Attributes**: No support for extended file attributes
5. **No i18n/l10n**: Output is ASCII-only where possible

## Error Handling

- **Exit Code 1**: Runtime errors (file not found, permission denied, etc.)
- **Exit Code 2**: Usage errors (invalid options, missing arguments)
- **No Error Messages on Success**: stdout is clean

## Output Format

- **No Colors**: Plain text output only
- **No Interactive Prompts**: Non-interactive by design
- **Deterministic Output**: Same input always produces same output

## What's Not Supported

- Environment variable behavior beyond basic cases
- Locale/encoding variations
- Signal handling
- Job control
- Terminal control
- Network operations
- Shell features (pipes, redirects, globbing - handled by shell)

## Testing Boundaries

Tests verify:
- Correct exit codes
- Correct stdout content
- Correct stderr content (error messages)
- Basic error handling

Tests do NOT verify:
- Timing/performance
- Memory usage (beyond obvious leaks)
- Edge cases in OS-level features

## Claims

This is NOT a GNU replacement. Commands are learning exercises and may lack features you expect from GNU coreutils.

## GVIBU/VIBIX support matrix

| Surface | Status | Evidence required |
|---|---|---|
| Linux Rust multicall | Supported for documented command specs | `cargo test`, parity tests |
| WASM writer path | Supported for writer-compatible commands | WASM build and wrapper checks |
| VIBIX `vibix-lib` raw/runtime layer | In progress; checked `echo` slice only | no-std build, runtime contract tests |
| VIBIX flat-binary integration | Compatibility path, not fully portable | VIBIX kernel plus QEMU command smoke |
| VIBIX canonical VFS 14/15 | Pending kernel registration handoff | kernel tests and QEMU verification |

VIBIX runtime errors are typed before use: negative errno encodings and
`u64::MAX` sentinels are not successful pointers, lengths, descriptors, or
process IDs. A non-empty checked write that makes zero progress is a runtime
error. Command semantics remain in GVIBU: normal output goes to stdout,
diagnostics go to stderr, runtime failures return status 1, and usage errors
return status 2 where specified.

The VIBIX runtime does not yet install a global allocator. It owns the checked
`brk` boundary and will not claim heap support until allocation lifetime and
resident-shell reuse are verified.

## Reference

- POSIX.1-2017: https://pubs.opengroup.org/onlinepubs/9699919799/
- GNU coreutils: https://www.gnu.org/software/coreutils/manual/
