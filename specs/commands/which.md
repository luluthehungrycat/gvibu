# which Spec

Overview
- Locate a command by searching directories in PATH.

Behavior
- Invocation: `gvibu which NAME...`
- For each NAME, search directories in the PATH environment variable.
- Print the full path of the first executable found, if any.
- If not found in any PATH directory, print nothing for that name.

Exit Codes
- 0: At least one name was found.
- 1: No names were found.

Output Conventions
- stdout: one full path per found name
- stderr: nothing by default

Implementation Notes
- Python: `os.environ.get("PATH", "")`, split by `:`, test with `os.access(path, os.X_OK)`.
- Rust: `env::var("PATH")`, split by `:`, test with `std::fs::metadata()` and permissions.
