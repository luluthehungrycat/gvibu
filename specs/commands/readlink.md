# readlink Spec

Overview
- Print the target of a symbolic link.

Behavior
- Invocation: `gvibu readlink <path>`
- Exactly one argument required (path to symlink).
- Prints the target of the symlink followed by a newline.
- If the path is not a symlink or doesn't exist, print error to stderr and exit 1.

Exit Codes
- 0: Success
- 1: Missing operand or read error

Output Conventions
- stdout: symlink target + newline
- stderr: error messages when applicable

Implementation Notes
- Use readlink() syscall via std::fs::read_link (Rust) or os.readlink (Python).
- Python and Rust implementations should mirror each other exactly.
