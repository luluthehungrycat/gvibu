"""dirname: strip last component from a file path."""

import sys


def run(args: list[str]) -> int:
    if not args:
        print("dirname: usage: dirname NAME", file=sys.stderr)
        return 2

    path = args[0]

    # Empty string -> current directory
    if not path:
        print(".")
        return 0

    # Strip trailing slashes
    stripped = path.rstrip("/")

    # If entirely slashes -> root
    if not stripped:
        print("/")
        return 0

    # Find last '/'
    last_slash = stripped.rfind("/")
    if last_slash == -1:
        # No slash: return current directory
        print(".")
    elif last_slash == 0:
        # Slash at start: root
        print("/")
    else:
        # Everything before the last slash
        print(stripped[:last_slash])

    return 0
