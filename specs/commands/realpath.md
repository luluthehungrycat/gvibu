# realpath Spec

Overview
- Print the canonicalized absolute pathname.

Behavior
- Invocation: `gvibu realpath <path>`
- Exactly one argument required (path to resolve).
- Resolves all symlinks and relative path components.
- Prints the canonical path followed by a newline.
- If the path doesn't exist, print error to stderr and exit 1.

Exit Codes
- 0: Success
- 1: Missing operand or resolution error

Output Conventions
- stdout: canonical path + newline
- stderr: error messages when applicable

Implementation Notes
- Use canonicalize() via std::fs::canonicalize (Rust) or os.path.realpath (Python).
- Python and Rust implementations should mirror each other exactly.
