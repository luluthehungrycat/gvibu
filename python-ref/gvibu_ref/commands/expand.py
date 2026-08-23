"""expand: convert tabs to spaces."""
from __future__ import annotations

import sys
from pathlib import Path


def parse_tab_stops(value: str) -> list[int]:
    stops: list[int] = []
    for part in value.split(","):
        try:
            stop = int(part)
        except ValueError as error:
            raise ValueError(f"invalid tab stop: {part}") from error
        if stop <= 0 or (stops and stop <= stops[-1]):
            raise ValueError(f"invalid tab stop: {part}")
        stops.append(stop)
    if not stops:
        raise ValueError("empty tab stop list")
    return stops


def spaces_to_next_tab(column: int, stops: list[int] | None) -> int:
    if stops is None:
        return 8 - (column % 8)
    for stop in stops:
        if stop > column:
            return stop - column
    interval = stops[-1] - stops[-2] if len(stops) >= 2 else stops[0]
    interval = max(interval, 1)
    offset = column - stops[-1]
    return interval - (offset % interval)


def expand_text(text: str, stops: list[int] | None, initial_only: bool) -> str:
    output: list[str] = []
    column = 0
    at_line_start = True
    for char in text:
        if char == "\n":
            output.append(char)
            column = 0
            at_line_start = True
        elif char == "\t" and (not initial_only or at_line_start):
            spaces = spaces_to_next_tab(column, stops)
            output.append(" " * spaces)
            column += spaces
        elif char == "\t":
            output.append(char)
        else:
            output.append(char)
            column += 1
            at_line_start = False
    return "".join(output)


def run(args: list[str]) -> int:
    stops: list[int] | None = None
    initial_only = False
    paths: list[str] = []
    parsing_options = True
    i = 0

    while i < len(args):
        arg = args[i]
        if parsing_options and arg == "--":
            parsing_options = False
        elif parsing_options and arg == "-i":
            initial_only = True
        elif parsing_options and arg == "-t":
            i += 1
            if i >= len(args):
                print("expand: option requires an argument: -t", file=sys.stderr)
                return 1
            try:
                stops = parse_tab_stops(args[i])
            except ValueError as error:
                print(f"expand: {error}", file=sys.stderr)
                return 1
        elif parsing_options and arg.startswith("-t") and len(arg) > 2:
            try:
                stops = parse_tab_stops(arg[2:])
            except ValueError as error:
                print(f"expand: {error}", file=sys.stderr)
                return 1
        elif parsing_options and arg.startswith("-"):
            print(f"expand: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            paths.append(arg)
        i += 1

    if not paths:
        try:
            sys.stdout.write(expand_text(sys.stdin.read(), stops, initial_only))
        except OSError as error:
            print(f"expand: error reading standard input: {error}", file=sys.stderr)
            return 1
        return 0

    exit_code = 0
    for path in paths:
        try:
            sys.stdout.write(expand_text(Path(path).read_text(), stops, initial_only))
        except OSError as error:
            print(f"expand: {path}: {error}", file=sys.stderr)
            exit_code = 1
    return exit_code
