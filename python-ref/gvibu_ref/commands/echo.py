"""echo: print arguments joined by spaces."""

import sys


def _interpret_escapes(s: str) -> str:
    """Interpret backslash escape sequences."""
    result = []
    i = 0
    while i < len(s):
        if s[i] == "\\" and i + 1 < len(s):
            nxt = s[i + 1]
            if nxt == "n":
                result.append("\n")
                i += 2
            elif nxt == "t":
                result.append("\t")
                i += 2
            elif nxt == "r":
                result.append("\r")
                i += 2
            elif nxt == "\\":
                result.append("\\")
                i += 2
            elif nxt == "'":
                result.append("'")
                i += 2
            elif nxt == '"':
                result.append('"')
                i += 2
            elif nxt == "0":
                # Octal escape: up to 3 digits
                octal = ""
                j = i + 2
                while len(octal) < 3 and j < len(s) and s[j].isdigit() and s[j] <= "7":
                    octal += s[j]
                    j += 1
                if octal:
                    result.append(chr(int(octal, 8)))
                    i = j
                else:
                    result.append("\\0")
                    i += 2
            else:
                result.append("\\" + nxt)
                i += 2
        else:
            result.append(s[i])
            i += 1
    return "".join(result)


def run(args: list[str]) -> int:
    newline = True
    escape_requested = False
    escape_disabled = False

    while args and args[0] in ("-n", "-e", "-E"):
        if args[0] == "-n":
            newline = False
        elif args[0] == "-e":
            escape_requested = True
        elif args[0] == "-E":
            escape_disabled = True
        args = args[1:]

    enable_escapes = escape_requested and not escape_disabled
    if enable_escapes:
        parts = [_interpret_escapes(a) for a in args]
    else:
        parts = args

    print(" ".join(parts), end="" if not newline else "\n")
    return 0
