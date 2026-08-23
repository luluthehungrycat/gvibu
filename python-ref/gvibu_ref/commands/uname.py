"""uname: print system information."""

import os
import sys


def run(args: list[str]) -> int:
    flags = set()
    all_flag = False

    for arg in args:
        if arg.startswith("-"):
            for ch in arg[1:]:
                if ch == "a":
                    all_flag = True
                elif ch in ("s", "n", "r", "v", "m", "o"):
                    flags.add(ch)
                else:
                    print(f"uname: invalid option: -{ch}", file=sys.stderr)
                    return 1
        else:
            print(f"uname: invalid option: {arg}", file=sys.stderr)
            return 1

    if not flags and not all_flag:
        flags.add("s")

    uname_info = os.uname()
    parts = []
    if all_flag or "s" in flags:
        parts.append(uname_info.sysname)
    if all_flag or "n" in flags:
        parts.append(uname_info.nodename)
    if all_flag or "r" in flags:
        parts.append(uname_info.release)
    if all_flag or "v" in flags:
        parts.append(uname_info.version)
    if all_flag or "m" in flags:
        parts.append(uname_info.machine)
    if all_flag or "o" in flags:
        parts.append("GNU/Linux" if uname_info.sysname == "Linux" else uname_info.sysname)

    print(" ".join(parts))
    return 0
