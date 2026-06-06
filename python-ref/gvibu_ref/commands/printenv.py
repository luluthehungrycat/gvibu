"""printenv: print all or part of environment."""

import os
import sys


def run(args: list[str]) -> int:
    if not args:
        for key, value in sorted(os.environ.items()):
            print(f"{key}={value}")
        return 0

    for name in args:
        print(os.environ.get(name, ""))
    return 0
