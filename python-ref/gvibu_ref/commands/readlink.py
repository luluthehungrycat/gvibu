import os
import sys


def run(args: list[str]) -> int:
    if len(args) != 1:
        print("readlink: missing operand", file=sys.stderr)
        return 1

    try:
        target = os.readlink(args[0])
        print(target)
        return 0
    except OSError as e:
        print(f"readlink: {args[0]}: {e}", file=sys.stderr)
        return 1
