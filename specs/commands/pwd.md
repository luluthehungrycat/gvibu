# pwd Spec

Overview
- Print the current working directory, followed by a newline.
- This is a minimal, deterministic implementation aligned with POSIX expectations for pwd in this project scope.

Behavior
- Invocation: `gvibu pwd`
- No options supported (single-character flags only; none are implemented for this command).
- If called with any operands (arguments), return exit code 2 (Usage error) and emit a brief usage message to stderr.
- When called with no arguments, output the absolute path of the current working directory on stdout, followed by a newline, and return exit code 0.

Exit Codes
- 0: Success
- 1: Runtime error
- 2: Usage error (invalid arguments)

Output Conventions
- stdout: current working directory path, newline-terminated
- stderr: error messages for usage or runtime errors

Edge Cases
- If the current directory cannot be determined, return exit code 1 and emit a reasonable error message on stderr.

Implementation Notes
- Implementations (Python and Rust) should mirror this spec exactly for parity.
