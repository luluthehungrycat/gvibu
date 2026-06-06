import sys


def run(args: list[str]) -> int:
    # Echo the provided arguments joined by spaces. If no args, print a blank line.
    # If the first argument is -n, suppress the trailing newline.
    newline = True
    if args and args[0] == "-n":
        newline = False
        args = args[1:]
    print(" ".join(args), end="" if not newline else "\n")
    return 0
