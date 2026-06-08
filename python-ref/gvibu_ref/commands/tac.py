"""tac: concatenate and write files in reverse."""

import sys


def run(args: list[str]) -> int:
    files = []
    for arg in args:
        if arg.startswith("-") and len(arg) > 1:
            print(f"tac: invalid option: {arg}", file=sys.stderr)
            return 1
        files.append(arg)

    exit_code = 0

    if not files:
        lines = sys.stdin.readlines()
        for line in reversed(lines):
            sys.stdout.write(line)
        return 0

    for fname in files:
        if fname == "-":
            lines = sys.stdin.readlines()
            for line in reversed(lines):
                sys.stdout.write(line)
        else:
            try:
                with open(fname) as f:
                    lines = f.readlines()
                for line in reversed(lines):
                    sys.stdout.write(line)
            except OSError as e:
                print(f"tac: {fname}: {e}", file=sys.stderr)
                exit_code = 1

    return exit_code
