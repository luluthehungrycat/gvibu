# head Spec

Overview
- Output the first part of files.
- Supports `-n` (line count), `-c` (byte count), and `-q` (quiet mode) options.

Behavior
- Invocation: `gvibu head [OPTION]... [FILE]...`
- Options:
  - `-n N`: print the first N lines (default: 10)
  - `-c N`: print the first N bytes
  - `-q`: never print file-name headers when reading multiple files
- `-n` and `-c` take a positive integer as their argument. N must be >= 0.
  - If N=0, print nothing.
  - If the file has fewer than N lines/bytes, print all content.
- `-n` and `-c` are mutually exclusive; if both are given, `-c` takes precedence.
- If no FILE is given, read from standard input.
- If multiple FILEs are given, print a header for each file.

Exit Codes
- 0: Success
- 1: Runtime error (file not found, read error, invalid N, missing argument)

Output Conventions
- stdout: the first N lines or N bytes of each file
- stderr: error messages for file errors when applicable
- With multiple files, include a header: `==> FILENAME <==` before each file's content

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
- For `-c`, read exactly N bytes (don't over-read). Use binary mode for accuracy.
- For `-n`, read line-by-line and stop after N lines; avoid reading the entire file.
