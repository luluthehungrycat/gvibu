# printenv Spec

Overview
- Print all or part of the environment.
- Minimal, read-only subset of env functionality.

Behavior
- Invocation: `gvibu printenv [NAME...]`
- If no arguments, prints all environment variables, one per line in `NAME=VALUE` format.
- If one or more NAMEs are given, prints the value of each, one per line.
- If a given NAME is not set in the environment, prints an empty line for it.
- Output order when printing all follows the system's environ ordering (arbitrary).

Exit Codes
- 0: Success
- 1: Runtime error

Output Conventions
- stdout: NAME=VALUE pairs (one per line) for the full environment, or VALUE (one per line) for requested variables.
- stderr: not used.
