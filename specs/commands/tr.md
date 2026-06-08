# tr Spec

Overview
- Translate or delete characters.

Behavior
- Invocation: `gvibu tr [options] <set1> [set2]`
- Reads from stdin, writes to stdout.
- -d: delete characters in set1.
- -s: squeeze repeated characters (replace runs with single occurrence).
- -c, -C: complement set1 (use all characters not in set1).
- Sets support range syntax: `a-z` expands to a through z.
- Sets support escape sequences: `\n`, `\t`, `\r`, `\\`, `\0`.
- Without -d, set2 provides replacement characters.
- If set2 is shorter than set1, the last character of set2 is repeated.

Exit Codes
- 0: Success
- 1: Missing operand or invalid option

Output Conventions
- stdout: translated output
- stderr: error messages when applicable
