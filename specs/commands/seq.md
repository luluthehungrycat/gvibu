# seq Spec

Overview
- Print a sequence of numbers.

Behavior
- Invocation: `gvibu seq [FIRST [STEP]] LAST`
- Print numbers from FIRST to LAST, incrementing by STEP, one per line.
- Default FIRST is 1, default STEP is 1.
- Numbers can be integers.
- If FIRST > LAST and STEP is positive, no output is produced.
- If invalid arguments, print error and exit 1.

Exit Codes
- 0: Success
- 1: Runtime error (invalid arguments)

Output Conventions
- stdout: one number per line
- stderr: error messages for invalid arguments

Implementation Notes
- Python: simple while loop.
- Rust: loop with step.
