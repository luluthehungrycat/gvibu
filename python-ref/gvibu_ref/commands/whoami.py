import os
import sys


def run(args: list[str]) -> int:
    if args:
        print("whoami: too many arguments", file=sys.stderr)
        return 1

    user = os.environ.get("USER") or os.environ.get("LOGNAME") or ""
    if not user:
        print("whoami: cannot find username", file=sys.stderr)
        return 1

    print(user)
    return 0
