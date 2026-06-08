import os
import sys


def run(args: list[str]) -> int:
    if args:
        print("logname: too many arguments", file=sys.stderr)
        return 1

    user = os.environ.get("LOGNAME") or os.environ.get("USER") or ""
    if not user:
        print("logname: no login name", file=sys.stderr)
        return 1

    print(user)
    return 0
