# uptime Spec

Overview
- Print the system uptime in human-readable format.

Behavior
- Invocation: `gvibu uptime`
- No arguments accepted; if any are given, print error to stderr and exit 1.
- Parses /proc/uptime and formats as:
  - "up X day(s), Y hour(s), Z minute(s)" for > 1 day
  - "up Y hour(s), Z minute(s)" for > 1 hour
  - "up Z minute(s)" for < 1 hour

Exit Codes
- 0: Success
- 1: Too many arguments

Output Conventions
- stdout: formatted uptime string + newline
- stderr: error messages when applicable

Implementation Notes
- Read /proc/uptime, parse first field as float number of seconds.
- Python and Rust implementations should mirror each other exactly.
