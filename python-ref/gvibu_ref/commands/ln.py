import os
import sys


def run(args: list[str]) -> int:
    symbolic = False
    force = False
    verbose = False
    targets = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-s":
            symbolic = True
        elif arg == "-f":
            force = True
        elif arg == "-v":
            verbose = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"ln: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            targets.append(arg)
        i += 1

    if len(targets) < 2:
        print("ln: missing operand", file=sys.stderr)
        return 1

    src, dst = targets[0], targets[1]

    if force and os.path.exists(dst) or os.path.islink(dst):
        try:
            os.remove(dst)
        except OSError:
            pass

    try:
        if symbolic:
            os.symlink(src, dst)
        else:
            os.link(src, dst)
        if verbose:
            print(f"linked '{dst}' -> '{src}'")
        return 0
    except OSError as e:
        print(f"ln: failed to create link '{dst}' -> '{src}': {e}", file=sys.stderr)
        return 1
