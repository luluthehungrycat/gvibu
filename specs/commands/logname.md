# logname Spec

Overview
- Print the login name of the current user.

Behavior
- Invocation: `gvibu logname`
- No arguments accepted; if any are given, print error to stderr and exit 1.
- Prints the login name followed by a newline.
- If the login name cannot be determined, print error to stderr and exit 1.

Exit Codes
- 0: Success
- 1: Too many arguments or no login name

Output Conventions
- stdout: login name + newline
- stderr: error messages when applicable

Implementation Notes
- Use $LOGNAME environment variable, with $USER as fallback.
- Python and Rust implementations should mirror each other exactly.
