"""shuf: randomly permute lines."""
import random
import sys


def run(args: list[str]) -> int:
    files = [a for a in args if not a.startswith('-')]

    lines = []
    if not files or files[0] == "-":
        lines = sys.stdin.read().splitlines(keepends=False)
    elif len(files) == 1:
        try:
            with open(files[0]) as f:
                lines = f.read().splitlines(keepends=False)
        except FileNotFoundError as e:
            print(f"shuf: {files[0]}: {e}", file=sys.stderr)
            return 1
    else:
        print("shuf: too many arguments", file=sys.stderr)
        return 1

    random.shuffle(lines)
    for line in lines:
        print(line)
    return 0
