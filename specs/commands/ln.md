# ln Spec

Overview
- Make links between files. Creates hard links by default, symbolic links with -s.

Behavior
- Invocation: `gvibu ln [options] <source> <dest>`
- Exactly two path arguments required (source and destination).
- Default: create a hard link.
- -s: create a symbolic link.
- -f: remove existing destination file before linking.
- -v: print diagnostic output for each linked file.

Exit Codes
- 0: Success
- 1: Missing operand, invalid option, or link creation error

Output Conventions
- stdout: verbose messages when -v is given
- stderr: error messages when applicable
