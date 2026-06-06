# head Spec

Overview
- Output the first part of files.
- Minimal, deterministic behavior per project scope.

Behavior
- Invocation: `gvibu head [OPTION]... [FILE]...`
- Options:
  - `-n N`: print the first N lines (default: 10)
- `-n` takes a positive integer as its argument. N must be >= 0.
  - If N=0, print nothing.
  - If the file has fewer than N lines, print all lines.
- If no FILE is given, read from standard input.
- If multiple FILEs are given, print a header for each file.

Exit Codes
- 0: Success
- 1: Runtime error (file not found, read error, invalid N)

Output Conventions
- stdout: the first N lines of each file
- stderr: error messages for file errors when applicable
- With multiple files, include a header: `==> FILENAME <==` before each file's content

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
- Read line-by-line and stop after N lines; avoid reading the entire file.
