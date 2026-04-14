# true Spec

Overview
- The true command exits with a status of 0 and produces no output.
- It accepts any number of arguments and ignores them.

Behavior
- Invocation: `gvibu true [ARGS...]`
- Any arguments are ignored; output is empty.
- Exit code is always 0 when called.

Exit Codes
- 0: Success (always)
- 1: Runtime error (not expected for this minimal spec)
- 2: Usage error (not applicable here)

Output Conventions
- stdout: nothing
- stderr: nothing

Implementation Notes
- Python and Rust implementations should always return 0, regardless of arguments.
