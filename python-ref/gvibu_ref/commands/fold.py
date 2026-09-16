"""fold: wrap each input line to fit in specified width."""
import sys


def run(args: list[str]) -> int:
    width = 80
    break_spaces = False

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-w":
            i += 1
            if i >= len(args):
                print("fold: option requires an argument: -w", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n <= 0:
                    raise ValueError
                width = n
            except ValueError:
                print(f"fold: invalid width: {args[i]}", file=sys.stderr)
                return 1
        elif arg == "-s":
            break_spaces = True
        elif arg.startswith('-') and len(arg) > 1:
            print(f"fold: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            print("fold: file arguments not supported", file=sys.stderr)
            return 1
        i += 1

    if width == 0:
        print("fold: width must be positive", file=sys.stderr)
        return 1

    for line_str in sys.stdin:
        line = line_str.rstrip('\n\r')
        remaining = line
        while remaining:
            if len(remaining) <= width:
                print(remaining)
                break

            if break_spaces:
                last_space = remaining.rfind(' ', 0, width)
                if last_space > 0:
                    split = last_space
                else:
                    split = width
            else:
                split = width

            chunk = remaining[:split]
            rest = remaining[split:]

            chunk = chunk.rstrip()
            if break_spaces:
                rest = rest.lstrip()

            print(chunk)
            remaining = rest

    return 0
