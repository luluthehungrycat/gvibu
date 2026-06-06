# dirname Spec

Overview
- Strip the last component from a file path, printing the directory portion.
- Minimal, deterministic behavior per project scope.

Behavior
- Invocation: `gvibu dirname NAME`
- Must provide exactly one NAME argument.
- Trailing slashes on NAME are ignored when determining the directory portion.
- If the path contains no `/` characters, output `.` (current directory).
- If the path consists entirely of slashes (e.g. `/`, `//`), output `/`.
- If the last `/` is at the start of the path (e.g. `/a`), output `/`.

Exit Codes
- 0: Success
- 1: Runtime error
- 2: Usage error (missing arguments)

Output Conventions
- stdout: the directory portion followed by a newline
- stderr: error messages for usage or runtime errors

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
- Handle empty string input by outputting `.` with exit code 0.
- Use string manipulation rather than filesystem calls — this is purely a string operation.
