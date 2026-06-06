import sys


def run(args: list[str]) -> int:
    append = False
    files: list[str] = []

    for arg in args:
        if arg == "-a":
            append = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"tee: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)

    outputs = [sys.stdout]  # Always write to stdout
    mode = "a" if append else "w"
    exit_code = 0

    for fname in files:
        try:
            f = open(fname, mode)
            outputs.append(f)
        except OSError as e:
            print(f"tee: {fname}: {e}", file=sys.stderr)
            exit_code = 1

    for line in sys.stdin:
        for out in outputs:
            try:
                out.write(line)
            except OSError as e:
                print(f"tee: write error: {e}", file=sys.stderr)
                exit_code = 1

    for out in outputs[1:]:  # Don't close stdout
        try:
            out.close()
        except OSError:
            pass

    return exit_code
