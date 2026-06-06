# yes Spec

Overview
- Output a string repeatedly until killed or write fails.
- Minimal, stdlib-only implementation per project scope.

Behavior
- Invocation: `gvibu yes [STRING...]`
- If no arguments, repeatedly prints "y" on its own line.
- If arguments are given, repeatedly prints them joined by spaces, followed by a newline.
- Runs indefinitely until SIGPIPE or write error.
- Write errors (broken pipe) are silently ignored.

Exit Codes
- 0: Not expected (does not exit normally)
- 1: Runtime error (not used in practice, defined for consistency)

Output Conventions
- stdout: repeated string + newline, forever
- stderr: not used

Implementation Notes
- Must handle SIGPIPE / write errors gracefully — loop terminates on write failure.
- Cannot be tested via shared parity checker (infinite loop). Test via subprocess piping.
