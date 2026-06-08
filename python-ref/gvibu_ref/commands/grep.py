"""grep: print lines matching a pattern."""

import sys
import os
import re


def run(args: list[str]) -> int:
    ignore_case = False
    recursive = False
    invert = False
    count = False
    show_line_numbers = False
    files_with_matches = False
    pattern = None
    paths = []
    i = 0

    while i < len(args):
        arg = args[i]
        if arg in ("-i", "--ignore-case"):
            ignore_case = True
        elif arg in ("-r", "-R", "--recursive"):
            recursive = True
        elif arg in ("-v", "--invert-match"):
            invert = True
        elif arg in ("-c", "--count"):
            count = True
        elif arg in ("-n", "--line-number"):
            show_line_numbers = True
        elif arg in ("-l", "--files-with-matches"):
            files_with_matches = True
        elif arg.startswith("-") and len(arg) > 1:
            for c in arg[1:]:
                if c == 'i':
                    ignore_case = True
                elif c in ('r', 'R'):
                    recursive = True
                elif c == 'v':
                    invert = True
                elif c == 'c':
                    count = True
                elif c == 'n':
                    show_line_numbers = True
                elif c == 'l':
                    files_with_matches = True
                else:
                    print(f"grep: invalid option: -{c}", file=sys.stderr)
                    return 1
        else:
            if pattern is None and not arg.startswith("-"):
                pattern = arg
            else:
                paths.append(arg)
        i += 1

    if pattern is None:
        print("grep: missing pattern", file=sys.stderr)
        return 1

    try:
        flags = re.IGNORECASE if ignore_case else 0
        regex = re.compile(pattern, flags)
    except re.error as e:
        print(f"grep: invalid pattern: {e}", file=sys.stderr)
        return 1

    if not paths and not recursive:
        return _grep_stream(sys.stdin, None, regex, invert, count, show_line_numbers, files_with_matches)

    exit_code = 1
    show_filename = len(paths) > 1 or recursive

    for path_str in paths:
        if os.path.isdir(path_str):
            if recursive:
                for root, dirs, files in os.walk(path_str):
                    for f in files:
                        fpath = os.path.join(root, f)
                        fname = fpath if show_filename else None
                        try:
                            with open(fpath, "r", errors="replace") as fh:
                                if _grep_stream(fh, fname, regex, invert, count, show_line_numbers, files_with_matches) == 0:
                                    exit_code = 0
                        except OSError:
                            pass
            else:
                print(f"grep: {path_str}: Is a directory", file=sys.stderr)
                exit_code = 1
        else:
            try:
                fname = path_str if show_filename else None
                if path_str == "-":
                    if _grep_stream(sys.stdin, fname, regex, invert, count, show_line_numbers, files_with_matches) == 0:
                        exit_code = 0
                else:
                    with open(path_str, "r", errors="replace") as fh:
                        if _grep_stream(fh, fname, regex, invert, count, show_line_numbers, files_with_matches) == 0:
                            exit_code = 0
            except OSError as e:
                print(f"grep: {path_str}: {e}", file=sys.stderr)
                exit_code = 1

    return exit_code


def _grep_stream(fh, filename, regex, invert, count, show_line_numbers, files_with_matches):
    match_count = 0
    matched = False

    for line_num, line in enumerate(fh, 1):
        is_match = bool(regex.search(line.rstrip("\n")))
        print_line = not is_match if invert else is_match

        if print_line:
            matched = True
            match_count += 1

            if files_with_matches:
                print(filename if filename else "(standard input)")
                return 0  # found match in this file

            if count:
                continue

            prefix = ""
            if filename:
                prefix = f"{filename}:"
            if show_line_numbers:
                prefix += f"{line_num}:"

            print(f"{prefix}{line}", end="")

    if count and matched:
        prefix = f"{filename}:" if filename else ""
        print(f"{prefix}{match_count}")

    return 0 if matched else 1
