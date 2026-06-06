"""cat: concatenate files and print to stdout."""

import sys


def run(args: list[str]) -> int:
    if not args:
        # Read from stdin
        try:
            for line in sys.stdin:
                sys.stdout.write(line)
        except OSError as e:
            print(f"cat: {e}", file=sys.stderr)
            return 1
        return 0

    exit_code = 0
    for filename in args:
        if filename == "-":
            try:
                for line in sys.stdin:
                    sys.stdout.write(line)
            except OSError as e:
                print(f"cat: {e}", file=sys.stderr)
                exit_code = 1
        else:
            try:
                with open(filename) as f:
                    for line in f:
                        sys.stdout.write(line)
            except OSError as e:
                print(f"cat: {filename}: {e}", file=sys.stderr)
                exit_code = 1

    return exit_code
