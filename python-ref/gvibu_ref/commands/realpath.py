import os
import sys


def run(args: list[str]) -> int:
    if len(args) != 1:
        print("realpath: missing operand", file=sys.stderr)
        return 1

    try:
        path = os.path.realpath(args[0])
        print(path)
        return 0
    except OSError as e:
        print(f"realpath: {args[0]}: {e}", file=sys.stderr)
        return 1
