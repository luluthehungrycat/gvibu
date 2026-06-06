"""wc: print newline, word, byte, and character counts."""

import sys


def _count_data(data: str) -> tuple[int, int, int, int]:
    """Count lines, words, bytes, and characters in data."""
    lines = data.count("\n")
    words = len(data.split()) if data else 0
    bytes_count = len(data.encode("utf-8"))
    chars = len(data)
    return lines, words, bytes_count, chars


def run(args: list[str]) -> int:
    # Parse flags
    flag_l = False
    flag_w = False
    flag_c = False
    flag_m = False

    files: list[str] = []
    i = 0
    while i < len(args):
        arg = args[i]
        if arg.startswith("-") and len(arg) > 1:
            for ch in arg[1:]:
                if ch == "l":
                    flag_l = True
                elif ch == "w":
                    flag_w = True
                elif ch == "c":
                    flag_c = True
                elif ch == "m":
                    flag_m = True
                else:
                    print(f"wc: invalid option: -{ch}", file=sys.stderr)
                    return 1
        else:
            files.append(arg)
        i += 1

    # Default: all four
    if not flag_l and not flag_w and not flag_c and not flag_m:
        flag_l = flag_w = flag_c = flag_m = True

    def fmt(lines: int, words: int, bytes_count: int, chars: int, name: str = "") -> str:
        parts = []
        if flag_l:
            parts.append(f"{lines:>7}")
        if flag_w:
            parts.append(f"{words:>7}")
        if flag_c:
            parts.append(f"{bytes_count:>7}")
        if flag_m:
            parts.append(f"{chars:>7}")
        if name:
            parts.extend([name])
        return " ".join(parts)

    exit_code = 0
    total_l = total_w = total_c = total_m = 0

    if not files:
        data = sys.stdin.read()
        l, w, c, m = _count_data(data)
        print(fmt(l, w, c, m))
        return 0

    for fname in files:
        try:
            with open(fname) as f:
                data = f.read()
        except OSError as e:
            print(f"wc: {fname}: {e}", file=sys.stderr)
            exit_code = 1
            continue

        l, w, c, m = _count_data(data)
        total_l += l
        total_w += w
        total_c += c
        total_m += m
        print(fmt(l, w, c, m, fname))

    if len(files) > 1:
        print(fmt(total_l, total_w, total_c, total_m, "total"))

    return exit_code
