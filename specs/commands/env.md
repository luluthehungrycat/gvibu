# env Spec

Overview
- Print or set environment variables.

Behavior
- Invocation: `gvibu env [NAME=VALUE]... [COMMAND [ARG]...]`
- With no arguments: print all environment variables, one per line (NAME=VALUE).
- With VAR=VAL assignments: set those variables before printing or running a command.
- With a command: run the specified command with the modified environment.
- Options:
  - `-i` or `--ignore-environment`: Start with an empty environment.
  - `-u` or `--unset VAR`: Remove a variable from the environment.
  - `-0` or `--null`: Separate printed environment entries with NUL bytes. This option cannot be used with a command.

Exit Codes
- 0: Success
- 1: Runtime error (if command execution fails)
- 125: `-0` was combined with a command
- 126: Command found but not executable
- 127: Command not found

Output Conventions
- stdout: environment variables (NAME=VALUE format, newline-delimited by default or NUL-delimited with `-0`) or output of executed command
- stderr: error messages

Implementation Notes
- Python: use `os.environ` and `subprocess` for running commands.
- Rust: use `std::env` and `std::process::Command`.
- Environment contents vary by platform; tests use an empty environment and explicit assignments when asserting exact output.
