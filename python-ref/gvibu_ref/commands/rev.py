"""rev: reverse the characters in each input line."""
from __future__ import annotations

import sys
from pathlib import Path


def reverse_lines(text: str) -> str:
    output: list[str] = []
    for segment in text.splitlines(keepends=True):
        if segment.endswith("\r\n"):
            output.append(segment[-3::-1] + "\r\n")
        elif segment.endswith("\n"):
            output.append(segment[-2::-1] + "\n")
        else:
            output.append(segment[::-1])
    return "".join(output)


def run(args: list[str]) -> int:
    paths: list[str] = []
    parsing_options = True
    for arg in args:
        if parsing_options and arg == "--":
            parsing_options = False
        elif parsing_options and arg.startswith("-"):
            print(f"rev: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            paths.append(arg)

    if not paths:
        try:
            sys.stdout.write(reverse_lines(sys.stdin.read()))
        except OSError as error:
            print(f"rev: error reading standard input: {error}", file=sys.stderr)
            return 1
        return 0

    exit_code = 0
    for path in paths:
        try:
            text = Path(path).read_text()
            sys.stdout.write(reverse_lines(text))
        except OSError as error:
            print(f"rev: {path}: {error}", file=sys.stderr)
            exit_code = 1
    return exit_code
