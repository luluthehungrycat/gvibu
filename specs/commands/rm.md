# rm Spec

Overview
- Remove files or directories.

Behavior
- Invocation: `gvibu rm [options] <file>...`
- At least one file argument required.
- -r, -R, --recursive: remove directories and their contents recursively.
- -f, --force: ignore nonexistent files, never prompt.
- -v, --verbose: print diagnostic output for each removed file.
- -d, --dir: remove empty directories and ordinary files; nonempty directories fail.
- -i, --interactive: prompt before each removal; -f overrides the prompt.
- --preserve-root: refuse an operand resolving to `/` (enabled by default).
- --no-preserve-root: disable the root-path guard.
- --: end option parsing so dash-prefixed names can be operands.
- With -f, errors for nonexistent files are suppressed.

Exit Codes
- 0: Success (all files removed)
- 1: Missing operand, invalid option, or removal error

Output Conventions
- stdout: verbose messages when -v is given
- stderr: error messages when applicable
