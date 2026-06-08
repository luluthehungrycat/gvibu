"""cat: concatenate files and print to stdout."""

import sys


def run(args: list[str]) -> int:
    number_lines = False
    files: list[str] = []

    for arg in args:
        if arg == "-n":
            number_lines = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"cat: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)

    line_num = [1]  # mutable counter so _print_lines can update it
    exit_code = 0

    if not files:
        _print_lines(sys.stdin, number_lines, line_num)
        return 0

    for fname in files:
        if fname == "-":
            _print_lines(sys.stdin, number_lines, line_num)
        else:
            try:
                with open(fname) as f:
                    _print_lines(f, number_lines, line_num)
            except OSError as e:
                print(f"cat: {fname}: {e}", file=sys.stderr)
                exit_code = 1

    return exit_code


def _print_lines(stream, number: bool, line_num: list):
    for line in stream:
        if number:
            sys.stdout.write(f"{line_num[0]:>6}\t{line}")
            line_num[0] += 1
        else:
            sys.stdout.write(line)
