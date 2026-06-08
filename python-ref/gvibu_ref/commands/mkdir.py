import os
import sys
import errno


def run(args: list[str]) -> int:
    parent = False
    dirs: list[str] = []

    for arg in args:
        if arg == "-p":
            parent = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"mkdir: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            dirs.append(arg)

    if not dirs:
        print("mkdir: missing operand", file=sys.stderr)
        return 1

    exit_code = 0
    for d in dirs:
        if parent:
            try:
                os.makedirs(d, exist_ok=True)
            except OSError as e:
                print(f"mkdir: cannot create directory '{d}': {e}", file=sys.stderr)
                exit_code = 1
        else:
            if os.path.exists(d):
                print(f"mkdir: cannot create directory '{d}': File exists", file=sys.stderr)
                exit_code = 1
            else:
                try:
                    os.mkdir(d)
                except OSError as e:
                    print(f"mkdir: cannot create directory '{d}': {e}", file=sys.stderr)
                    exit_code = 1

    return exit_code
