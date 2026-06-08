# uniq Spec

Overview
- Filter adjacent duplicate lines from input.

Behavior
- Invocation: `gvibu uniq [OPTIONS]`
- Reads from stdin.
- Without options, prints each line once per adjacent run (suppresses consecutive duplicates).
- Options:
  - `-u`: Only print unique lines (lines that are not repeated)
  - `-d`: Only print repeated lines
  - `-c`: Prefix each output line with its count (right-aligned, width 4)

Exit Codes
- 0: Success
- 1: Invalid option

Output Conventions
- stdout: filtered lines + newline for each
- stderr: error messages when applicable

Implementation Notes
- Read all lines, track adjacent runs with counts, filter based on flags.
- Python and Rust implementations should mirror each other exactly.
