# tee Spec

Overview
- Read from standard input and write to standard output and files.

Behavior
- Invocation: `gvibu tee [OPTION]... [FILE]...`
- Options:
  - `-a`: append to the given FILEs, do not overwrite
- Read standard input until EOF.
- Write the input to stdout and to each specified FILE.
- If a file cannot be opened for writing, print error to stderr and continue.

Exit Codes
- 0: Success
- 1: Write error

Output Conventions
- stdout: copy of stdin
- stderr: error messages when applicable

Implementation Notes
- Write to stdout and all files simultaneously (line-by-line or chunk-by-chunk).
- Python and Rust implementations should mirror each other exactly.
