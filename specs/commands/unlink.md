# unlink Spec

Overview
- Remove a file.
- Equivalent to `rm` for a single file.

Behavior
- Invocation: `gvibu unlink FILE`
- Removes the named file.
- Exactly one argument required.
- Prints error to stderr and exits 1 on failure.

Exit Codes
- 0: Success
- 1: Wrong number of arguments or file removal failed

Output Conventions
- stdout: nothing
- stderr: error messages when applicable
