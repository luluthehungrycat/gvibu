import os
import sys


def run(args: list[str]) -> int:
    if not args:
        print("rmdir: missing operand", file=sys.stderr)
        return 1

    exit_code = 0
    for d in args:
        try:
            os.rmdir(d)
        except OSError as e:
            print(f"rmdir: failed to remove '{d}': {e}", file=sys.stderr)
            exit_code = 1

    return exit_code
