"""sleep: delay for a specified number of seconds."""

import sys
import time


def run(args: list[str]) -> int:
    if not args:
        print("sleep: usage: sleep NUMBER", file=sys.stderr)
        return 2

    try:
        seconds = int(args[0])
    except ValueError:
        print(f"sleep: invalid number: {args[0]}", file=sys.stderr)
        return 2

    if seconds < 0:
        print(f"sleep: invalid number: {args[0]}", file=sys.stderr)
        return 1

    if seconds > 0:
        time.sleep(seconds)
    return 0
