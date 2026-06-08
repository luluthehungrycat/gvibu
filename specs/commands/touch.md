# touch Spec

Overview
- Update file timestamps or create empty files.
- Supports `-a` (access time) and `-m` (modification time) flags.

Behavior
- Invocation: `gvibu touch [OPTIONS] FILE...`
- If FILE does not exist, create it as an empty file.
- If FILE exists, update its timestamps based on options.
- Options:
  - `-a`: change only access time (preserve modification time)
  - `-m`: change only modification time (preserve access time)
  - (no flag): change both access and modification times (default)
  - `-am` or `-a -m`: change both (same as default)
- Options must precede FILE arguments.
- Must accept at least one FILE argument.

Exit Codes
- 0: Success (all files touched)
- 1: Runtime error (if any file operation fails, invalid option)
- 2: Usage error (missing arguments)

Output Conventions
- stdout: nothing on success
- stderr: error messages for file operation failures or usage errors

Implementation Notes
- Python: use `os.utime()` with `None` for current time, or pass existing atime/mtime to preserve.
- Rust: use `std::fs::File` with `set_accessed()` / `set_modified()`.
