import sys


def run(args: list[str]) -> int:
    show_unique = False
    show_repeated = False
    show_count = False

    for arg in args:
        if arg == "-u":
            show_unique = True
        elif arg == "-d":
            show_repeated = True
        elif arg == "-c":
            show_count = True
        else:
            print(f"uniq: invalid option -- '{arg}'", file=sys.stderr)
            return 1

    lines = [line.rstrip("\n") for line in sys.stdin]
    if not lines:
        return 0

    output = []
    current = lines[0]
    count = 1

    for line in lines[1:]:
        if line == current:
            count += 1
        else:
            output.append((count, current))
            current = line
            count = 1
    output.append((count, current))

    for c, line in output:
        print_it = True
        if show_unique and not show_repeated:
            print_it = c == 1
        elif show_repeated and not show_unique:
            print_it = c > 1

        if print_it:
            if show_count:
                print(f"{c:>4} {line}")
            else:
                print(line)

    return 0
