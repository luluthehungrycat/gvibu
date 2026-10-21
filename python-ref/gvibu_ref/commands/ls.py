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
    append_indicator = False
    list_directories_themselves = False
    recursive = False
    show_inode = False
    color_mode = "never"
    paths = []
    parsing_options = True

    i = 0
    while i < len(args):
        arg = args[i]
        if parsing_options and arg == "--":
            parsing_options = False
        elif parsing_options and arg == "-a":
            show_all = True
        elif parsing_options and arg == "-A":
            show_almost_all = True
        elif parsing_options and arg == "-l":
            long = True
        elif parsing_options and arg == "-h":
            human = True
        elif parsing_options and arg == "-t":
            sort_time = True
        elif parsing_options and arg == "-r":
            reverse = True
        elif parsing_options and arg == "-S":
            sort_size = True
        elif parsing_options and arg == "-1":
            one_per_line = True
        elif parsing_options and arg == "--color":
            color_mode = "auto"
        elif parsing_options and arg.startswith("--color="):
            color_mode = arg.split("=", 1)[1]
            if color_mode not in ("auto", "always", "never"):
                print(f"ls: invalid color option: {color_mode}", file=sys.stderr)
                return 1
        elif parsing_options and arg.startswith("-") and len(arg) > 1:
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
                elif c == 'F':
                    append_indicator = True
                elif c == 'd':
                    list_directories_themselves = True
                elif c == 'R':
                    recursive = True
                elif c == 'i':
                    show_inode = True
                else:
                    print(f"ls: invalid option: -{c}", file=sys.stderr)
                    return 1
        else:
            paths.append(arg)
        i += 1

    color = color_mode == "always" or (color_mode == "auto" and sys.stdout.isatty())

    if not paths:
        paths.append(".")

    exit_code = 0
    first = True

    for path in paths:
        try:
            meta = os.lstat(path) if list_directories_themselves else os.stat(path)
        except OSError as e:
            print(f"ls: cannot access '{path}': {e}", file=sys.stderr)
            exit_code = 1
            continue

        if stat.S_ISDIR(meta.st_mode) and not list_directories_themselves:
            if not first:
                print()
            if len(paths) > 1:
                print(f"{path}:")
            first = False
            ec = _list_directory(path, long, show_all, show_almost_all, human, sort_time, reverse, sort_size, color, append_indicator, recursive, show_inode)
            if ec:
                exit_code = ec
            if len(paths) > 1:
                print()
        else:
            if not first:
                print()
            first = False
            if long:
                long_name = path + (_indicator(meta) if append_indicator else "")
                output = _format_long(long_name, meta, human)
                if show_inode:
                    output = f"{meta.st_ino} {output}"
                print(output)
            else:
                name = _display_name(path)
                if show_inode:
                    name = f"{meta.st_ino} {name}"
                if append_indicator:
                    name += _indicator(meta)
                print(_colorize(name, meta) if color else name)

    return exit_code


def _list_directory(dir_path: str, long: bool, show_all: bool, show_almost_all: bool,
                    human: bool, sort_time: bool, reverse: bool, sort_size: bool,
                    color: bool = False, append_indicator: bool = False,
                    recursive: bool = False, show_inode: bool = False) -> int:
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
            meta = os.lstat(full)
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
                long_name = name + (_indicator(meta) if append_indicator else "")
                output = _format_long(long_name, meta, human)
                if show_inode:
                    output = f"{meta.st_ino} {output}"
                print(output)
    else:
        for name, meta in items:
            if show_inode and meta:
                name = f"{meta.st_ino} {name}"
            if append_indicator and meta:
                name += _indicator(meta)
            print(_colorize(name, meta) if color and meta else name)

    exit_code = 0
    if recursive:
        for name, meta in items:
            if meta and stat.S_ISDIR(meta.st_mode):
                subdir = name if dir_path == "." else os.path.join(dir_path, name)
                print()
                print(f"{subdir}:")
                exit_code |= _list_directory(subdir, long, show_all, show_almost_all, human, sort_time, reverse, sort_size,
                                             color, append_indicator, recursive, show_inode)
    return exit_code


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


def _indicator(meta: os.stat_result) -> str:
    if stat.S_ISDIR(meta.st_mode):
        return "/"
    if stat.S_ISLNK(meta.st_mode):
        return "@"
    if meta.st_mode & 0o111:
        return "*"
    return ""


def _colorize(name: str, meta: os.stat_result) -> str:
    if stat.S_ISDIR(meta.st_mode):
        return f"\x1b[34m{name}\x1b[0m"
    if stat.S_ISLNK(meta.st_mode):
        return f"\x1b[36m{name}\x1b[0m"
    if meta.st_mode & 0o111:
        return f"\x1b[32m{name}\x1b[0m"
    return name
