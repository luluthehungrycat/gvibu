import os
import sys


def run(args: list[str]) -> int:
    if len(args) != 2:
        print("link: exactly two arguments required: FILE LINK", file=sys.stderr)
        return 1

    src, dst = args[0], args[1]

    if not os.path.exists(src):
        print(f"link: {src}: no such file or directory", file=sys.stderr)
        return 1

    try:
        os.link(src, dst)
    except OSError as e:
        print(f"link: cannot create link {dst} -> {src}: {e}", file=sys.stderr)
        return 1

    return 0
