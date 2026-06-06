"""seq: print a sequence of numbers."""

import sys


def run(args: list[str]) -> int:
    if not args:
        print("seq: usage: seq [FIRST [STEP]] LAST", file=sys.stderr)
        return 1

    try:
        if len(args) == 1:
            first = 1
            step = 1
            last = int(args[0])
        elif len(args) == 2:
            first = int(args[0])
            step = 1
            last = int(args[1])
        elif len(args) == 3:
            first = int(args[0])
            step = int(args[1])
            last = int(args[2])
        else:
            print("seq: too many arguments", file=sys.stderr)
            return 1
    except ValueError:
        print("seq: invalid number", file=sys.stderr)
        return 1

    if step > 0:
        if first > last:
            return 0
        i = first
        while i <= last:
            print(i)
            i += step
    elif step < 0:
        if first < last:
            return 0
        i = first
        while i >= last:
            print(i)
            i += step
    else:
        # step == 0
        print("seq: step cannot be zero", file=sys.stderr)
        return 1

    return 0
