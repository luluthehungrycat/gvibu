# cut Spec

Overview
- Remove sections from each line of files.

Behavior
- Invocation: `gvibu cut [options] [file...]`
- -d <delimiter>: use DELIM instead of TAB for field delimiter.
- -f <list>: select only these fields (1-indexed, comma/range syntax).
- -b <list>: select only these bytes (1-indexed, comma/range syntax).
- -c <list>: select only these characters (1-indexed, comma/range syntax).
- -s: suppress lines with no delimiter (for -f) or lines shorter than specified positions (for -b, -c).
- List syntax: `1,3-5,7` selects positions 1, 3, 4, 5, 7.
- Only one of -f, -b, or -c can be specified at a time.
- If no file is given, reads from stdin.
- Prints selected content joined by the delimiter (for -f) or as concatenated content (for -b, -c).

Exit Codes
- 0: Success
- 1: Invalid option, missing argument, or file error

Output Conventions
- stdout: selected content
- stderr: error messages when applicable
