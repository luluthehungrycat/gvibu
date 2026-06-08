"""tail: output the last part of files."""

import sys
import os


def run(args: list[str]) -> int:
    num_lines = 10
    num_bytes = None
    files = []
    i = 0

    while i < len(args):
        arg = args[i]
        if arg == "-n":
            i += 1
            if i >= len(args):
                print("tail: option requires an argument: -n", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n < 0:
                    print("tail: invalid number of lines: 0", file=sys.stderr)
                    return 1
                num_lines = n
            except ValueError:
                print(f"tail: invalid number of lines: {args[i]}", file=sys.stderr)
                return 1
        elif arg == "-c":
            i += 1
            if i >= len(args):
                print("tail: option requires an argument: -c", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n < 0:
                    print("tail: invalid number of bytes: 0", file=sys.stderr)
                    return 1
                num_bytes = n
            except ValueError:
                print(f"tail: invalid number of bytes: {args[i]}", file=sys.stderr)
                return 1
        elif arg.startswith("-") and len(arg) > 1:
            print(f"tail: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)
        i += 1

    if not files:
        if num_bytes is not None:
            data = sys.stdin.buffer.read()
            sys.stdout.buffer.write(data[-num_bytes:])
        else:
            lines = sys.stdin.readlines()
            for line in lines[-num_lines:]:
                sys.stdout.write(line)
        return 0

    exit_code = 0
    for fname in files:
        if fname == "-":
            if num_bytes is not None:
                data = sys.stdin.buffer.read()
                sys.stdout.buffer.write(data[-num_bytes:])
            else:
                lines = sys.stdin.readlines()
                for line in lines[-num_lines:]:
                    sys.stdout.write(line)
        else:
            try:
                if num_bytes is not None:
                    with open(fname, "rb") as f:
                        f.seek(0, os.SEEK_END)
                        file_len = f.tell()
                        if file_len <= num_bytes:
                            f.seek(0)
                            sys.stdout.buffer.write(f.read())
                        else:
                            f.seek(-num_bytes, os.SEEK_END)
                            sys.stdout.buffer.write(f.read())
                else:
                    with open(fname) as f:
                        lines = f.readlines()
                    for line in lines[-num_lines:]:
                        sys.stdout.write(line)
            except OSError as e:
                print(f"tail: {fname}: {e}", file=sys.stderr)
                exit_code = 1

    return exit_code
