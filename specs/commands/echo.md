# echo Spec

Overview
- Print its arguments joined by spaces, followed by a newline.
- Minimal, deterministic behavior aligned with project scope.

Behavior
- Invocation: `gvibu echo [-n] [ARGS...]`
- No long options; only simple positional arguments supported.
- If the first argument is `-n`, no trailing newline is printed.
- If called with a mix of quotes or spaces, arguments are passed as-is and joined by spaces in output.
- If called with no arguments, prints a blank line (just newline), unless `-n` is the only argument (prints nothing).

Exit Codes
- 0: Success
- 1: Runtime error
- 2: Usage error (not expected in this minimal scope, but defined for parity consistency)

Output Conventions
- stdout: arguments joined by spaces (or a newline if no args), optionally without trailing newline when `-n` is used
- stderr: error messages for usage or runtime errors when applicable

Examples

    $ gvibu echo hello world
    hello world

    $ gvibu echo
    (newline only)

    $ gvibu echo -n hello
    hello$

Implementation Notes
- Implementations (Python and Rust) should mirror this spec exactly for parity.
