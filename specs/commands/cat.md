# cat Spec

Overview
- Concatenate files and print to standard output.
- Supports `-n` flag for line numbering.

Behavior
- Invocation: `gvibu cat [OPTIONS] [FILE]...`
- If no FILE is given, read from standard input.
- If FILE is `-`, read from standard input at that position.
- Print the contents of each FILE to stdout in order.
- Options:
  - `-n`: number output lines, starting from 1, right-aligned in 6 columns with a tab separator.

Exit Codes
- 0: Success
- 1: Runtime error (file not found, read error, invalid option)

Output Conventions
- stdout: contents of the file(s) or stdin, with optional line numbers
- stderr: error messages for file errors when applicable

Implementation Notes
- Python and Rust implementations should mirror this spec exactly for parity.
- When `-n` is used, line numbers should be padded to 6 characters right-aligned, followed by a tab.
- Line numbering continues across file boundaries (like GNU cat).
