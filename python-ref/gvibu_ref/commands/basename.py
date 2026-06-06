import os
import sys


def run(args: list[str]) -> int:
    # Basename requires at least NAME
    if len(args) == 0 or (len(args) == 1 and args[0] == ""):
        print("basename: usage: basename NAME [SUFFIX]", file=sys.stderr)
        return 1
    name = args[0]
    # Strip any trailing slashes to correctly identify the last component
    name = name.rstrip("/")
    # Extract the last path component
    basename = name.split("/")[-1] if "/" in name else name
    if len(args) > 1:
        suffix = args[1]
        if suffix and basename.endswith(suffix):
            basename = basename[: -len(suffix)]
    print(basename)
    return 0
