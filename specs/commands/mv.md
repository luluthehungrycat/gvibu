# mv Spec

Overview
- Move (rename) files.

Behavior
- Invocation: `gvibu mv [options] <source> <dest>`
- Exactly two path arguments required.
- -i: prompt before overwrite (y/yes to confirm).
- -f: do not prompt before overwriting (overrides -i when both given, but -i takes precedence if after -f).
- -v: print diagnostic output for each moved file.
- -n: do not overwrite an existing file.

Exit Codes
- 0: Success
- 1: Missing operand, invalid option, or move error

Output Conventions
- stdout: verbose messages when -v is given
- stderr: error messages when applicable
