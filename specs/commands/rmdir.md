# rmdir Spec

Overview
- Remove empty directories.

Behavior
- Invocation: `gvibu rmdir [OPTION]... DIRECTORY...`
- Removes each specified empty directory.
- Fails if a directory does not exist, is not empty, or is not a directory.
- Options: none (no -p, no --ignore-fail-on-non-empty for minimal scope)

Exit Codes
- 0: All directories removed successfully
- 1: Any directory removal failed

Output Conventions
- stdout: nothing
- stderr: error messages when applicable
