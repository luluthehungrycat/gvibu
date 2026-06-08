"""du: estimate file space usage."""

import os
import sys


def run(args: list[str]) -> int:
    human = False
    summary = False
    paths = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-h":
            human = True
        elif arg == "-s":
            summary = True
        elif arg.startswith("-") and len(arg) > 1:
            for c in arg[1:]:
                if c == "h":
                    human = True
                elif c == "s":
                    summary = True
                else:
                    print(f"du: invalid option: -{c}", file=sys.stderr)
                    return 1
        else:
            paths.append(arg)
        i += 1

    if not paths:
        paths.append(".")

    grand_total = 0
    exit_code = 0

    for path_str in paths:
        try:
            entries = _du_walk(path_str, summary)
            for size, p in entries:
                display = path_str if p == "." else p
                if human:
                    print(f"{_human_size(size)}\t{display}")
                else:
                    print(f"{size}\t{display}")
            if entries:
                grand_total += entries[-1][0]
        except OSError as e:
            print(f"du: {path_str}: {e}", file=sys.stderr)
            exit_code = 1

    if grand_total > 0 and len(paths) > 1:
        if human:
            print(f"{_human_size(grand_total)}\ttotal")
        else:
            print(f"{grand_total}\ttotal")

    return exit_code


def _du_walk(path: str, summary: bool) -> list[tuple[int, str]]:
    meta = os.lstat(path)
    entries: list[tuple[int, str]] = []

    if os.path.isdir(path) and not os.path.islink(path) and not summary:
        try:
            for entry in os.listdir(path):
                sub_path = os.path.join(path, entry)
                sub_meta = os.lstat(sub_path)
                if os.path.isdir(sub_path) and not os.path.islink(sub_path):
                    sub_entries = _du_walk(sub_path, False)
                    entries.extend(sub_entries)
                else:
                    entries.append((sub_meta.st_size, sub_path))
        except PermissionError as e:
            print(f"du: {path}: {e}", file=sys.stderr)

        entries.sort(key=lambda x: x[1])
        dir_size = sum(s for s, _ in entries)
        entries.append((dir_size, path))
    else:
        entries.append((meta.st_size, path))

    return entries


def _human_size(bytes_val: int) -> str:
    units = ["", "K", "M", "G", "T", "P"]
    size = float(bytes_val)
    unit_idx = 0

    while size >= 1024.0 and unit_idx < len(units) - 1:
        size /= 1024.0
        unit_idx += 1

    if unit_idx == 0:
        return str(bytes_val)
    elif size >= 100.0:
        return f"{size:.0f}{units[unit_idx]}"
    elif size >= 10.0:
        return f"{size:.1f}{units[unit_idx]}"
    else:
        return f"{size:.1f}{units[unit_idx]}"
