# whoami Spec

Overview
- Print the effective username of the current user.

Behavior
- Invocation: `gvibu whoami`
- No arguments accepted; if any are given, print error to stderr and exit 1.
- Prints the username followed by a newline.
- If the username cannot be determined, print error to stderr and exit 1.

Exit Codes
- 0: Success
- 1: Too many arguments or cannot find username

Output Conventions
- stdout: the username + newline
- stderr: error messages when applicable

Implementation Notes
- Use $USER environment variable, with $LOGNAME as fallback.
- Python and Rust implementations should mirror each other exactly.
