"""head: output the first part of files."""

import sys


def _print_lines(lines: list[str], num_lines: int) -> None:
    """Print up to num_lines from the list."""
    for line in lines[:num_lines]:
        sys.stdout.write(line)


def _print_bytes(data: bytes, num_bytes: int) -> None:
    """Print up to num_bytes from the data."""
    sys.stdout.buffer.write(data[:num_bytes])


def run(args: list[str]) -> int:
    num_lines = 10
    num_bytes: int | None = None
    quiet = False
    files: list[str] = []
    i = 0

    while i < len(args):
        arg = args[i]
        if arg == "-n":
            i += 1
            if i >= len(args):
                print("head: option requires an argument: -n", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n < 0:
                    print("head: invalid number of lines: 0", file=sys.stderr)
                    return 1
                num_lines = n
            except ValueError:
                print(f"head: invalid number of lines: {args[i]}", file=sys.stderr)
                return 1
        elif arg == "-q":
            quiet = True
        elif arg == "-c":
            i += 1
            if i >= len(args):
                print("head: option requires an argument: -c", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n < 0:
                    print("head: invalid number of bytes: 0", file=sys.stderr)
                    return 1
                num_bytes = n
            except ValueError:
                print(f"head: invalid number of bytes: {args[i]}", file=sys.stderr)
                return 1
        elif arg.startswith("-") and len(arg) > 1:
            print(f"head: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)
        i += 1

    exit_code = 0

    if not files:
        # Read from stdin
        if num_bytes is not None:
            data = sys.stdin.buffer.read(num_bytes)
            _print_bytes(data, num_bytes)
        else:
            lines = []
            try:
                for line in sys.stdin:
                    lines.append(line)
                    if len(lines) >= num_lines:
                        break
            except OSError as e:
                print(f"head: {e}", file=sys.stderr)
                return 1
            _print_lines(lines, num_lines)
        return 0

    for idx, fname in enumerate(files):
        if len(files) > 1 and not quiet:
            if idx > 0:
                sys.stdout.write("\n")
            sys.stdout.write(f"==> {fname} <==\n")

        if fname == "-":
            if num_bytes is not None:
                data = sys.stdin.buffer.read(num_bytes)
                _print_bytes(data, num_bytes)
            else:
                lines = []
                try:
                    for line in sys.stdin:
                        lines.append(line)
                        if len(lines) >= num_lines:
                            break
                except OSError as e:
                    print(f"head: {e}", file=sys.stderr)
                    exit_code = 1
                _print_lines(lines, num_lines)
        else:
            try:
                if num_bytes is not None:
                    with open(fname, "rb") as f:
                        data = f.read(num_bytes)
                    _print_bytes(data, num_bytes)
                else:
                    with open(fname) as f:
                        lines = []
                        for line in f:
                            lines.append(line)
                            if len(lines) >= num_lines:
                                break
                    _print_lines(lines, num_lines)
            except OSError as e:
                print(f"head: {fname}: {e}", file=sys.stderr)
                exit_code = 1

    return exit_code
