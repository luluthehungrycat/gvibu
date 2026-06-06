# sleep Spec

Overview
- Delay for a specified number of seconds.
- Minimal, integer-only implementation.

Behavior
- Invocation: `gvibu sleep NUMBER`
- NUMBER must be a non-negative integer.
- If NUMBER is 0, returns immediately.
- Sleeps for NUMBER seconds (as integer), then exits with 0.
- Fractional seconds are not supported.

Exit Codes
- 0: Success
- 1: Runtime error (e.g., negative number)
- 2: Usage error (missing argument, invalid format)

Output Conventions
- stdout: none (silent).
- stderr: usage/error messages when applicable.

Implementation Notes
- Only integer seconds are supported (no suffix like `-m` or `-h`).
- Both Python and Rust implementations should use `time.sleep` / `std::thread::sleep`.
