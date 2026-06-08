# seq Spec

Overview
- Print a sequence of numbers.

Behavior
- Invocation: `gvibu seq [OPTION]... [FIRST [STEP]] LAST`
- Options:
  - `-s STRING`: use STRING as separator between numbers (default: newline)
  - `-w`: equal width by padding with leading zeros
- Options must precede positional arguments.
- Print numbers from FIRST to LAST, incrementing by STEP.
- Default FIRST is 1, default STEP is 1.
- Numbers can be integers.
- If FIRST > LAST and STEP is positive, no output is produced.
- If invalid arguments, print error and exit 1.

Exit Codes
- 0: Success
- 1: Runtime error (invalid arguments)

Output Conventions
- stdout: numbers separated by the separator string; final newline always present
- stderr: error messages for invalid arguments

Implementation Notes
- Python and Rust implementations should mirror each other exactly.
- `-w` width is determined by the longest number as a decimal string.
