# wc Spec

Overview
- Print newline, word, and byte counts for each file.
- Minimal, deterministic behavior per project scope.

Behavior
- Invocation: `gvibu wc [OPTION]... [FILE]...`
- Options:
  - `-l`: print only the newline count
  - `-w`: print only the word count
  - `-c`: print only the byte count
  - `-m`: print only the character count (UTF-8 aware)
- When multiple options are given, output is in the order: lines, words, bytes, chars.
- If no options are given, default to `-lwcm` (all four counts).
- A word is a maximal non-whitespace sequence; whitespace is space, tab, and newline.
- If no FILE is given, read from standard input (displayed as no filename or as `-`).
- If multiple FILEs are given, print per-file counts followed by a total line.

Exit Codes
- 0: Success
- 1: Runtime error (file not found, read error)

Output Conventions
- stdout: formatted as columns of counts followed by the filename (or blank for stdin)
- stderr: error messages for file errors when applicable

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
- Count bytes (not characters) for `-c`.
- Count Unicode code points (characters) for `-m` via `data.chars().count()` in Rust and `len(data)` in Python.
- The last line without a trailing newline still counts as one line if it contains data.
