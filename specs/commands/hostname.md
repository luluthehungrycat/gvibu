# hostname Spec

Overview
- Print the system hostname.

Behavior
- Invocation: `gvibu hostname`
- No arguments accepted; if any are given, print error to stderr and exit 1.
- Prints the hostname followed by a newline.
- If the hostname cannot be determined, print error to stderr and exit 1.

Exit Codes
- 0: Success
- 1: Too many arguments or cannot determine hostname

Output Conventions
- stdout: hostname + newline
- stderr: error messages when applicable

Implementation Notes
- Read /etc/hostname, with /proc/sys/kernel/hostname as fallback.
- Python and Rust implementations should mirror each other exactly.
