"""grep: print lines matching a pattern."""

import sys
import os
import re


def run(args: list[str]) -> int:
    options = {
        "ignore_case": False,
        "recursive": False,
        "invert": False,
        "count": False,
        "line_numbers": False,
        "files_with_matches": False,
        "fixed": False,
        "word": False,
        "quiet": False,
        "only_matching": False,
        "no_messages": False,
        "after": 0,
        "before": 0,
    }
    pattern = None
    paths = []
    parsing_options = True
    i = 0

    while i < len(args):
        arg = args[i]
        if parsing_options and arg == "--":
            parsing_options = False
        elif parsing_options and arg in ("-A", "-B", "-C", "--after-context", "--before-context", "--context"):
            i += 1
            if i >= len(args):
                print(f"grep: option requires an argument: {arg}", file=sys.stderr)
                return 1
            try:
                amount = int(args[i])
                if amount < 0:
                    raise ValueError
            except ValueError:
                print(f"grep: invalid context length argument: {args[i]}", file=sys.stderr)
                return 1
            if arg in ("-A", "--after-context"):
                options["after"] = amount
            elif arg in ("-B", "--before-context"):
                options["before"] = amount
            else:
                options["before"] = options["after"] = amount
        elif parsing_options and arg.startswith("-") and arg != "-":
            if arg in ("-i", "--ignore-case"):
                options["ignore_case"] = True
            elif arg in ("-r", "-R", "--recursive"):
                options["recursive"] = True
            elif arg in ("-v", "--invert-match"):
                options["invert"] = True
            elif arg in ("-c", "--count"):
                options["count"] = True
            elif arg in ("-n", "--line-number"):
                options["line_numbers"] = True
            elif arg in ("-l", "--files-with-matches"):
                options["files_with_matches"] = True
            elif arg in ("-E", "--extended-regexp"):
                pass
            elif arg in ("-F", "--fixed-strings"):
                options["fixed"] = True
            elif arg in ("-w", "--word-regexp"):
                options["word"] = True
            elif arg in ("-q", "--quiet"):
                options["quiet"] = True
            elif arg in ("-o", "--only-matching"):
                options["only_matching"] = True
            elif arg in ("-s", "--no-messages"):
                options["no_messages"] = True
            elif len(arg) > 2 and arg[1] in "ABC" and arg[2:].isdigit():
                amount = int(arg[2:])
                options[{"A": "after", "B": "before", "C": "before"}[arg[1]]] = amount
                if arg[1] == "C":
                    options["after"] = amount
            elif len(arg) > 2 and all(c in "ivcnlFEwqos" for c in arg[1:]):
                for flag in arg[1:]:
                    key = {"i": "ignore_case", "v": "invert", "c": "count", "n": "line_numbers", "l": "files_with_matches", "F": "fixed", "w": "word", "q": "quiet", "o": "only_matching", "s": "no_messages"}.get(flag)
                    if key:
                        options[key] = True
            else:
                print(f"grep: invalid option: {arg}", file=sys.stderr)
                return 1
        elif pattern is None:
            pattern = arg
        else:
            paths.append(arg)
        i += 1

    if pattern is None:
        print("grep: missing pattern", file=sys.stderr)
        return 1

    expression = re.escape(pattern) if options["fixed"] else pattern
    if options["word"]:
        expression = rf"(?<![A-Za-z0-9_])(?:{expression})(?![A-Za-z0-9_])"
    try:
        regex = re.compile(expression, re.IGNORECASE if options["ignore_case"] else 0)
    except re.error as error:
        print(f"grep: invalid pattern: {error}", file=sys.stderr)
        return 1

    if not paths and not options["recursive"]:
        return _grep_stream(sys.stdin, None, regex, options)

    exit_code = 1
    show_filename = len(paths) > 1 or options["recursive"]
    for path_str in paths:
        if os.path.isdir(path_str):
            if options["recursive"]:
                for root, _, files in os.walk(path_str):
                    for name in files:
                        fpath = os.path.join(root, name)
                        try:
                            with open(fpath, "r", errors="replace") as stream:
                                if _grep_stream(stream, fpath if show_filename else None, regex, options) == 0:
                                    exit_code = 0
                        except OSError:
                            if not options["no_messages"]:
                                print(f"grep: {fpath}: cannot read file", file=sys.stderr)
            elif not options["no_messages"]:
                print(f"grep: {path_str}: Is a directory", file=sys.stderr)
        elif path_str == "-":
            if _grep_stream(sys.stdin, None, regex, options) == 0:
                exit_code = 0
        else:
            try:
                with open(path_str, "r", errors="replace") as stream:
                    if _grep_stream(stream, path_str if show_filename else None, regex, options) == 0:
                        exit_code = 0
            except OSError as error:
                if not options["no_messages"]:
                    print(f"grep: {path_str}: {error}", file=sys.stderr)

    return exit_code


def _grep_stream(stream, filename, regex, options):
    lines = [line.rstrip("\n") for line in stream]
    selected = [bool(regex.search(line)) != options["invert"] for line in lines]
    match_count = sum(selected)
    matched = match_count > 0

    if options["quiet"] and matched:
        return 0
    if options["files_with_matches"] and matched:
        if filename:
            print(filename)
        return 0
    if options["count"]:
        prefix = f"{filename}:" if filename else ""
        print(f"{prefix}{match_count}")
        return 0 if matched else 1

    def emit(text, line_number):
        prefix = f"{filename}:" if filename else ""
        if options["line_numbers"]:
            prefix += f"{line_number}:"
        print(f"{prefix}{text}")

    if options["only_matching"]:
        for index, line in enumerate(lines):
            if not selected[index]:
                continue
            for match in regex.finditer(line):
                if match.start() != match.end():
                    emit(match.group(0), index + 1)
        return 0 if matched else 1

    ranges = []
    for index, is_selected in enumerate(selected):
        if is_selected:
            start = max(0, index - options["before"])
            end = min(len(lines) - 1, index + options["after"])
            if ranges and start <= ranges[-1][1] + 1:
                ranges[-1] = (ranges[-1][0], max(ranges[-1][1], end))
            else:
                ranges.append((start, end))

    previous_end = None
    for start, end in ranges:
        if (options["before"] or options["after"]) and previous_end is not None and start > previous_end + 1:
            print("--")
        for index in range(start, end + 1):
            emit(lines[index], index + 1)
        previous_end = end
    return 0 if matched else 1
