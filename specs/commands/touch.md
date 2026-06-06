# touch Spec

Overview
- Update file timestamps or create empty files.
- Minimal deterministic behavior per project scope.

Behavior
- Invocation: `gvibu touch FILE...`
- If FILE does not exist, create it as an empty file.
- If FILE exists, update its access and modification timestamps to the current time.
- Must accept at least one FILE argument.

Exit Codes
- 0: Success (all files touched)
- 1: Runtime error (if any file operation fails)
- 2: Usage error (missing arguments)

Output Conventions
- stdout: nothing on success
- stderr: error messages for file operation failures or usage errors

Implementation Notes
- Python: use `os.utime()` and `os.close(os.open(...))` or `pathlib.Path.touch()`.
- Rust: use `std::fs::File::create()` for new files, `std::fs::File::open()` and set times for existing.
