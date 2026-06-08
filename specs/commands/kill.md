# kill Spec

Overview
- Terminate a process by sending a signal.

Behavior
- Invocation: `gvibu kill [options] <pid>` or `gvibu kill -<signal> <pid>`
- Default signal is SIGTERM (15).
- `-l`: list all signal names and numbers.
- `-s <signal> <pid>`: send specified signal.
- `-<signal> <pid>`: shorthand for -s (e.g., `-9` or `-KILL`).
- Signal can be a number or name (case-insensitive, with or without SIG prefix).

Exit Codes
- 0: Success (signal sent)
- 1: Missing operand, invalid signal/pid, or kill error

Output Conventions
- stdout: signal list when -l is given
- stderr: error messages when applicable

Implementation Notes
- Uses libc::kill (Rust) or os.kill (Python).
