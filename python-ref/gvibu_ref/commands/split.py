"""split: split a file into pieces."""

import sys
import os


def run(args: list[str]) -> int:
    lines = -1
    bytes = -1
    numeric = False
    suffix_len = 2
    files = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-d":
            numeric = True
        elif arg == "-a":
            i += 1
            if i >= len(args):
                print("split: option requires an argument: -a", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n < 1:
                    raise ValueError
                suffix_len = n
            except ValueError:
                print(f"split: invalid suffix length: {args[i]}", file=sys.stderr)
                return 1
        elif arg == "-l":
            i += 1
            if i >= len(args):
                print("split: option requires an argument: -l", file=sys.stderr)
                return 1
            try:
                n = int(args[i])
                if n <= 0:
                    raise ValueError
                lines = n
            except ValueError:
                print(f"split: invalid number of lines: {args[i]}", file=sys.stderr)
                return 1
        elif arg == "-b":
            i += 1
            if i >= len(args):
                print("split: option requires an argument: -b", file=sys.stderr)
                return 1
            try:
                n = parse_size(args[i])
                if n <= 0:
                    raise ValueError
                bytes = n
            except ValueError:
                print(f"split: invalid number of bytes: {args[i]}", file=sys.stderr)
                return 1
        elif arg.startswith("-") and len(arg) > 1:
            print(f"split: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)
        i += 1

    if lines == -1 and bytes == -1:
        lines = 1000

    input_file = files[0] if files else None
    prefix = files[1] if len(files) > 1 else "x"

    try:
        if input_file and input_file != "-":
            with open(input_file) as f:
                if bytes > 0:
                    split_bytes(f, prefix, bytes, numeric, suffix_len)
                else:
                    split_lines(f, prefix, lines, numeric, suffix_len)
        else:
            if bytes > 0:
                split_bytes(sys.stdin, prefix, bytes, numeric, suffix_len)
            else:
                split_lines(sys.stdin, prefix, lines, numeric, suffix_len)
    except OSError as e:
        print(f"split: {e}", file=sys.stderr)
        return 1

    return 0


def parse_size(s: str) -> int:
    s = s.strip()
    if not s:
        raise ValueError
    suffix = s[-1]
    if suffix in ("b", "k", "m", "g"):
        mult = {"b": 512, "k": 1024, "m": 1024 * 1024, "g": 1024 * 1024 * 1024}[suffix]
        num = int(s[:-1])
        return num * mult
    return int(s)


def split_lines(f, prefix: str, max_lines: int, numeric: bool, suffix_len: int):
    file_num = 0
    line_count = 0
    current_file = None

    for line in f:
        if line_count >= max_lines:
            if current_file:
                current_file.close()
            current_file = None
            line_count = 0
            file_num += 1

        if current_file is None:
            name = make_suffix(prefix, file_num, numeric, suffix_len)
            current_file = open(name, "w")

        current_file.write(line)
        line_count += 1

    if current_file:
        current_file.close()


def split_bytes(f, prefix: str, max_bytes: int, numeric: bool, suffix_len: int):
    file_num = 0
    byte_count = 0
    current_file = None

    for line in f:
        for ch in line:
            if byte_count >= max_bytes:
                if current_file:
                    current_file.close()
                current_file = None
                byte_count = 0
                file_num += 1

            if current_file is None:
                name = make_suffix(prefix, file_num, numeric, suffix_len)
                current_file = open(name, "w")

            current_file.write(ch)
            byte_count += 1

    if current_file:
        current_file.close()


def make_suffix(prefix: str, num: int, numeric: bool, length: int) -> str:
    if numeric:
        return f"{prefix}{num:0{length}d}"
    else:
        # Alphabetic: aa, ab, ... zz, aaa, aab ...
        n = num
        suffix = []
        while True:
            suffix.append(chr(ord("a") + (n % 26)))
            n //= 26
            if n == 0:
                break
        while len(suffix) < length:
            suffix.append("a")
        suffix.reverse()
        return prefix + "".join(suffix)
