"""nl: number lines of files."""


def run(args: list[str]) -> int:
    files: list[str] = []
    body_start = 1

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-v":
            i += 1
            if i >= len(args):
                print("nl: option requires an argument: -v", file=__import__("sys").stderr)
                return 1
            try:
                body_start = int(args[i])
            except ValueError:
                print(f"nl: invalid starting line number: {args[i]}", file=__import__("sys").stderr)
                return 1
        elif arg.startswith("-") and len(arg) > 1:
            print(f"nl: invalid option: {arg}", file=__import__("sys").stderr)
            return 1
        else:
            files.append(arg)
        i += 1

    line_num = body_start
    exit_code = 0

    if not files:
        try:
            exit_code = _number_lines(__import__("sys").stdin, line_num)
        except OSError as e:
            print(f"nl: stdin: {e}", file=__import__("sys").stderr)
            exit_code = 1
    else:
        for fname in files:
            if fname == "-":
                try:
                    exit_code |= _number_lines(__import__("sys").stdin, line_num)
                except OSError as e:
                    print(f"nl: stdin: {e}", file=__import__("sys").stderr)
                    exit_code = 1
            else:
                try:
                    with open(fname) as f:
                        exit_code |= _number_lines(f, line_num)
                except OSError as e:
                    print(f"nl: {fname}: {e}", file=__import__("sys").stderr)
                    exit_code = 1

    return exit_code


def _number_lines(reader, line_num: int) -> int:
    for line in reader:
        line = line.rstrip("\n")
        if not line:
            print()
        else:
            print(f"{line_num:>6}\t{line}")
            line_num += 1
    return 0
