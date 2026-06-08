import os
import sys


def run(args: list[str]) -> int:
    interactive = False
    force = False
    verbose = False
    no_clobber = False
    targets = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-i":
            interactive = True
        elif arg == "-f":
            force = True
        elif arg == "-v":
            verbose = True
        elif arg == "-n":
            no_clobber = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"mv: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            targets.append(arg)
        i += 1

    if len(targets) < 2:
        print("mv: missing operand", file=sys.stderr)
        return 1

    if interactive and force:
        force = False

    src, dst = targets[0], targets[1]

    if no_clobber and os.path.exists(dst):
        return 0

    if interactive and os.path.exists(dst):
        response = input(f"mv: overwrite '{dst}'? ")
        if response.lower() not in ("y", "yes"):
            return 0

    try:
        os.rename(src, dst)
        if verbose:
            print(f"renamed '{src}' -> '{dst}'")
        return 0
    except OSError as e:
        print(f"mv: cannot move '{src}' to '{dst}': {e}", file=sys.stderr)
        return 1
