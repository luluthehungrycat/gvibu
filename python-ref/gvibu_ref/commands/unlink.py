import os
import sys


def run(args: list[str]) -> int:
    if len(args) != 1:
        print("unlink: exactly one argument required: FILE", file=sys.stderr)
        return 1

    path = args[0]

    try:
        os.unlink(path)
    except OSError as e:
        print(f"unlink: cannot unlink {path}: {e}", file=sys.stderr)
        return 1

    return 0
