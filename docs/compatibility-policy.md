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

## Reference

- POSIX.1-2017: https://pubs.opengroup.org/onlinepubs/9699919799/
- GNU coreutils: https://www.gnu.org/software/coreutils/manual/
