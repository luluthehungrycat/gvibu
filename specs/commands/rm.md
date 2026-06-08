# rm Spec

Overview
- Remove files or directories.

Behavior
- Invocation: `gvibu rm [options] <file>...`
- At least one file argument required.
- -r, -R, --recursive: remove directories and their contents recursively.
- -f, --force: ignore nonexistent files, never prompt.
- -v, --verbose: print diagnostic output for each removed file.
- With -f, errors for nonexistent files are suppressed.

Exit Codes
- 0: Success (all files removed)
- 1: Missing operand, invalid option, or removal error

Output Conventions
- stdout: verbose messages when -v is given
- stderr: error messages when applicable
