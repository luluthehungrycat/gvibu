import os
import sys
import shutil


def run(args: list[str]) -> int:
    recursive = False
    force = False
    verbose = False
    targets = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg in ("-r", "-R", "--recursive"):
            recursive = True
        elif arg in ("-f", "--force"):
            force = True
        elif arg in ("-v", "--verbose"):
            verbose = True
        elif arg in ("-rf", "-fr", "-Rf", "-fR"):
            recursive = True
            force = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"rm: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            targets.append(arg)
        i += 1

    if not targets:
        print("rm: missing operand", file=sys.stderr)
        return 1

    exit_code = 0

    for target in targets:
        try:
            if recursive:
                if os.path.isdir(target):
                    shutil.rmtree(target)
                else:
                    os.remove(target)
            else:
                os.remove(target)
            if verbose:
                print(f"removed '{target}'")
        except OSError as e:
            if not force:
                print(f"rm: cannot remove '{target}': {e}", file=sys.stderr)
                exit_code = 1

    return exit_code
