"""seq: print a sequence of numbers."""

import sys


def _parse_args(args: list[str]):
    """Parse seq arguments. Returns (first, step, last, separator, equal_width)."""
    separator = "\n"
    equal_width = False
    pos_args: list[str] = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-w":
            equal_width = True
            i += 1
        elif arg == "-s":
            i += 1
            if i >= len(args):
                print("seq: option requires an argument: -s", file=sys.stderr)
                return None
            separator = args[i]
            i += 1
        elif arg.startswith("-") and len(arg) > 1:
            # Combined flags
            for j, ch in enumerate(arg[1:]):
                if ch == "w":
                    equal_width = True
                elif ch == "s":
                    rest = arg[j + 2 :]
                    if not rest:
                        i += 1
                        if i >= len(args):
                            print("seq: option requires an argument: -s", file=sys.stderr)
                            return None
                        separator = args[i]
                    else:
                        separator = rest
                    break
                else:
                    print(f"seq: invalid option: -{ch}", file=sys.stderr)
                    return None
            i += 1
        else:
            pos_args.append(arg)
            i += 1

    if not pos_args:
        print("seq: usage: seq [FIRST [STEP]] LAST", file=sys.stderr)
        return None

    try:
        if len(pos_args) == 1:
            first = 1
            step = 1
            last = int(pos_args[0])
        elif len(pos_args) == 2:
            first = int(pos_args[0])
            step = 1
            last = int(pos_args[1])
        elif len(pos_args) == 3:
            first = int(pos_args[0])
            step = int(pos_args[1])
            last = int(pos_args[2])
        else:
            print("seq: too many arguments", file=sys.stderr)
            return None
    except ValueError:
        print("seq: invalid number", file=sys.stderr)
        return None

    if step == 0:
        print("seq: step cannot be zero", file=sys.stderr)
        return None

    return first, step, last, separator, equal_width


def _generate_numbers(first: int, step: int, last: int) -> list[int]:
    nums: list[int] = []
    if step > 0:
        i = first
        while i <= last:
            nums.append(i)
            i += step
    else:
        i = first
        while i >= last:
            nums.append(i)
            i += step
    return nums


def _format_numbers(nums: list[int], separator: str, equal_width: bool) -> str:
    if not nums:
        return ""

    if equal_width:
        width = max(len(str(n)) for n in nums)
        strs = [str(n).zfill(width) for n in nums]
    else:
        strs = [str(n) for n in nums]

    return separator.join(strs) + "\n"


def run(args: list[str]) -> int:
    parsed = _parse_args(args)
    if parsed is None:
        return 1

    first, step, last, separator, equal_width = parsed

    if (step > 0 and first > last) or (step < 0 and first < last):
        return 0

    nums = _generate_numbers(first, step, last)
    sys.stdout.write(_format_numbers(nums, separator, equal_width))
    return 0
