"""touch: update file timestamps or create empty files."""

import os
import sys


def run(args: list[str]) -> int:
    if not args:
        print("touch: usage: touch FILE...", file=sys.stderr)
        return 2

    flag_a = False
    flag_m = False
    files: list[str] = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg.startswith("-") and len(arg) > 1:
            for ch in arg[1:]:
                if ch == "a":
                    flag_a = True
                elif ch == "m":
                    flag_m = True
                else:
                    print(f"touch: invalid option: -{ch}", file=sys.stderr)
                    return 1
        else:
            files.append(arg)
        i += 1

    if not files:
        print("touch: usage: touch FILE...", file=sys.stderr)
        return 2

    if not flag_a and not flag_m:
        flag_a = True
        flag_m = True

    exit_code = 0
    for fname in files:
        try:
            # Create if doesn't exist
            with open(fname, "a"):
                pass

            if flag_a and flag_m:
                os.utime(fname, None)
            elif flag_a:
                # Update access time only, preserve modification time
                mtime = os.path.getmtime(fname)
                os.utime(fname, (None, mtime))
            elif flag_m:
                # Update modification time only, preserve access time
                atime = os.path.getatime(fname)
                os.utime(fname, (atime, None))
        except OSError as e:
            print(f"touch: {fname}: {e}", file=sys.stderr)
            exit_code = 1

    return exit_code
