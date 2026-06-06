# cat Spec

Overview
- Concatenate files and print to standard output.
- Minimal, deterministic behavior per project scope.

Behavior
- Invocation: `gvibu cat [FILE]...`
- If no FILE is given, read from standard input.
- If FILE is `-`, read from standard input at that position.
- Print the contents of each FILE to stdout in order.
- No flags or options are supported in this minimal scope.

Exit Codes
- 0: Success
- 1: Runtime error (file not found, read error)

Output Conventions
- stdout: contents of the file(s) or stdin
- stderr: error messages for file errors when applicable

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
- Read and write in buffered chunks rather than loading entire files into memory.
- When reading from stdin, read line-by-line and write to stdout.
