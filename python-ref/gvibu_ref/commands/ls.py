"""ls: list directory contents."""

import sys
import os
import stat
import pwd
import grp
from datetime import datetime


def run(args: list[str]) -> int:
    show_all = False
    show_almost_all = False
    long = False
    human = False
    sort_time = False
    reverse = False
    sort_size = False
    one_per_line = True
    paths = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-a":
            show_all = True
        elif arg == "-A":
            show_almost_all = True
        elif arg == "-l":
            long = True
        elif arg == "-h":
            human = True
        elif arg == "-t":
            sort_time = True
        elif arg == "-r":
            reverse = True
        elif arg == "-S":
            sort_size = True
        elif arg == "-1":
            one_per_line = True
        elif arg.startswith("-") and len(arg) > 1:
            for c in arg[1:]:
                if c == 'a':
                    show_all = True
                elif c == 'A':
                    show_almost_all = True
                elif c == 'l':
                    long = True
                elif c == 'h':
                    human = True
                elif c == 't':
                    sort_time = True
                elif c == 'r':
                    reverse = True
                elif c == 'S':
                    sort_size = True
                elif c == '1':
                    one_per_line = True
                else:
                    print(f"ls: invalid option: -{c}", file=sys.stderr)
                    return 1
        else:
            paths.append(arg)
        i += 1

    if not paths:
        paths.append(".")

    exit_code = 0
    first = True

    for path in paths:
        try:
            meta = os.stat(path)
        except OSError as e:
            print(f"ls: cannot access '{path}': {e}", file=sys.stderr)
            exit_code = 1
            continue

        if stat.S_ISDIR(meta.st_mode):
            if not first:
                print()
            if len(paths) > 1:
                print(f"{path}:")
            first = False
            ec = _list_directory(path, long, show_all, show_almost_all, human, sort_time, reverse, sort_size)
            if ec:
                exit_code = ec
            if len(paths) > 1:
                print()
        else:
            if not first:
                print()
            first = False
            if long:
                print(_format_long(path, meta, human))
            else:
                print(_display_name(path))

    return exit_code


def _list_directory(dir_path: str, long: bool, show_all: bool, show_almost_all: bool,
                    human: bool, sort_time: bool, reverse: bool, sort_size: bool) -> int:
    try:
        entries = os.listdir(dir_path)
    except OSError as e:
        print(f"ls: cannot read directory '{dir_path}': {e}", file=sys.stderr)
        return 1

    items = []
    for name in entries:
        if name.startswith("."):
            if show_all:
                pass
            elif show_almost_all and name not in (".", ".."):
                pass
            else:
                continue
        full = os.path.join(dir_path, name)
        try:
            meta = os.stat(full)
            items.append((name, meta))
        except OSError:
            items.append((name, None))

    if sort_time:
        items.sort(key=lambda x: x[1].st_mtime if x[1] else 0, reverse=True)
    elif sort_size:
        items.sort(key=lambda x: x[1].st_size if x[1] else 0, reverse=True)
    else:
        items.sort(key=lambda x: x[0])

    if reverse:
        items.reverse()

    if long:
        for name, meta in items:
            if meta:
                print(_format_long(name, meta, human))
    else:
        for name, _ in items:
            print(name)

    return 0


def _format_long(name: str, meta: os.stat_result, human: bool) -> str:
    file_type = 'd' if stat.S_ISDIR(meta.st_mode) else 'l' if stat.S_ISLNK(meta.st_mode) else '-'
    perms = _format_mode(meta.st_mode)
    nlink = meta.st_nlink
    try:
        owner = pwd.getpwuid(meta.st_uid).pw_name
    except KeyError:
        owner = str(meta.st_uid)
    try:
        group = grp.getgrgid(meta.st_gid).gr_name
    except KeyError:
        group = str(meta.st_gid)
    size = _human_size(meta.st_size) if human else str(meta.st_size)
    mtime = _format_time(meta.st_mtime)
    return f"{file_type}{perms} {nlink:>2} {owner} {group} {size:>8} {mtime} {name}"


def _format_mode(mode: int) -> str:
    def _perm(bits: int) -> str:
        r = 'r' if mode & bits[0] else '-'
        w = 'w' if mode & bits[1] else '-'
        x = 'x' if mode & bits[2] else '-'
        return r + w + x
    return (_perm([0o400, 0o200, 0o100]) +
            _perm([0o040, 0o020, 0o010]) +
            _perm([0o004, 0o002, 0o001]))


def _human_size(bytes: int) -> str:
    units = ["", "K", "M", "G", "T"]
    size = float(bytes)
    idx = 0
    while size >= 1024 and idx < len(units) - 1:
        size /= 1024
        idx += 1
    if idx == 0:
        return str(bytes)
    return f"{size:.1f}{units[idx]}"


def _format_time(secs: float) -> str:
    dt = datetime.fromtimestamp(secs)
    now = datetime.now()
    if dt.year == now.year:
        return dt.strftime("%b %e %H:%M")
    else:
        return dt.strftime("%b %e  %Y")


def _display_name(path: str) -> str:
    return path.rsplit("/", 1)[-1] if "/" in path else path
