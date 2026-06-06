"""touch: update file timestamps or create empty files."""

import os
import sys


def run(args: list[str]) -> int:
    if not args:
        print("touch: usage: touch FILE...", file=sys.stderr)
        return 2

    exit_code = 0
    for fname in args:
        try:
            # Create if doesn't exist; update timestamps if it does
            with open(fname, "a"):
                os.utime(fname, None)
        except OSError as e:
            print(f"touch: {fname}: {e}", file=sys.stderr)
            exit_code = 1

    return exit_code
