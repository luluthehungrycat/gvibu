# link Spec

Overview
- Create a hard link to a file.
- Equivalent to `ln` without `-s` for a single link.

Behavior
- Invocation: `gvibu link FILE LINK`
- Creates a hard link named LINK pointing to FILE.
- Exactly two arguments required: source file and link name.
- Prints error to stderr and exits 1 on failure.

Exit Codes
- 0: Success
- 1: Wrong number of arguments or link creation failed

Output Conventions
- stdout: nothing
- stderr: error messages when applicable
