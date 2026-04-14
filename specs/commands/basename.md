# basename Spec

Overview
- Extract the final component of a path, optionally stripping a suffix.
- Minimal, deterministic behavior per project scope.

Behavior
- Invocation: `gvibu basename NAME [SUFFIX]`
- Must provide at least NAME; SUFFIX is optional and, if provided, will be removed from the basename if present.
- Trailing slashes on NAME are ignored when extracting the basename.
- If SUFFIX is provided and matches the end of the basename, that suffix is removed.

Exit Codes
- 0: Success
- 1: Runtime error
- 2: Usage error (missing arguments)

Output Conventions
- stdout: the basename (with suffix removed when applicable) followed by a newline
- stderr: error messages for usage or runtime errors

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
