# cut Spec

Overview
- Remove sections from each line of files.

Behavior
- Invocation: `gvibu cut [options] [file...]`
- -d <delimiter>: use DELIM instead of TAB for field delimiter.
- -f <list>: select only these fields (1-indexed, comma/range syntax).
- List syntax: `1,3-5,7` selects fields 1, 3, 4, 5, 7.
- If no file is given, reads from stdin.
- Prints selected fields joined by the delimiter.

Exit Codes
- 0: Success
- 1: Invalid option, missing argument, or file error

Output Conventions
- stdout: selected fields
- stderr: error messages when applicable
