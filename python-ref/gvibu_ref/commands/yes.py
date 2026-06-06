"""yes: output a string repeatedly."""

import sys


def run(args: list[str]) -> int:
    if args:
        text = " ".join(args)
    else:
        text = "y"

    try:
        while True:
            sys.stdout.write(text + "\n")
            sys.stdout.flush()
    except (BrokenPipeError, OSError):
        sys.stderr.close()
        return 0
