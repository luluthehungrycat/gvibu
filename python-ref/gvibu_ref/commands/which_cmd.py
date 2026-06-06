"""which: locate a command by searching PATH."""

import os
import sys


def run(args: list[str]) -> int:
    if not args:
        return 1

    path = os.environ.get("PATH", "")
    directories = [d for d in path.split(":") if d]
    found = False

    for name in args:
        for directory in directories:
            full_path = os.path.join(directory, name)
            if os.path.isfile(full_path) and os.access(full_path, os.X_OK):
                print(full_path)
                found = True
                break

    return 0 if found else 1
