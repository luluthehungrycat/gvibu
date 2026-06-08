"""printf: format and print data."""

import sys
import re


def run(args: list[str]) -> int:
    if not args:
        print("printf: missing operand", file=sys.stderr)
        return 1

    fmt = interpret_escapes(args[0])
    fmt_args = args[1:] if len(args) > 1 else [""]

    result = []
    i = 0
    arg_idx = 0
    fmt_len = len(fmt)

    while i < fmt_len:
        if fmt[i] == '\\':
            # Already handled by interpret_escapes for most sequences
            result.append(fmt[i])
            i += 1
            continue

        if fmt[i] == '%':
            i += 1
            # Parse width
            width_str = ""
            while i < fmt_len and fmt[i].isdigit():
                width_str += fmt[i]
                i += 1
            width = int(width_str) if width_str else 0

            if i >= fmt_len:
                break

            spec = fmt[i]
            arg = fmt_args[arg_idx % len(fmt_args)]
            arg_idx += 1

            if spec == '%':
                result.append('%')
            elif spec == 's':
                if width and len(arg) < width:
                    result.append(arg.rjust(width))
                else:
                    result.append(arg)
            elif spec in ('d', 'i'):
                try:
                    val = int(arg)
                except ValueError:
                    val = 0
                if width:
                    result.append(f"{val:{width}d}")
                else:
                    result.append(str(val))
            elif spec == 'u':
                try:
                    val = int(arg)
                    if val < 0:
                        val = 0
                except ValueError:
                    val = 0
                if width:
                    result.append(f"{val:{width}d}")
                else:
                    result.append(str(val))
            elif spec == 'o':
                try:
                    val = int(arg)
                except ValueError:
                    val = 0
                if width:
                    result.append(f"{val:{width}o}")
                else:
                    result.append(oct(val)[2:])
            elif spec == 'x':
                try:
                    val = int(arg)
                except ValueError:
                    val = 0
                if width:
                    result.append(f"{val:{width}x}")
                else:
                    result.append(hex(val)[2:])
            elif spec == 'X':
                try:
                    val = int(arg)
                except ValueError:
                    val = 0
                if width:
                    result.append(f"{val:{width}X}")
                else:
                    result.append(hex(val)[2:].upper())
            elif spec == 'c':
                ch = arg[0] if arg else ' '
                if width and width > 1:
                    result.append(ch.rjust(width))
                else:
                    result.append(ch)
            elif spec in ('f', 'e', 'E', 'g', 'G'):
                try:
                    val = float(arg)
                except ValueError:
                    val = 0.0
                if width:
                    result.append(f"{val:{width}}")
                else:
                    result.append(str(val))
            else:
                result.append(f"%{spec}")

            i += 1
            continue

        result.append(fmt[i])
        i += 1

    sys.stdout.write("".join(result))
    return 0


def interpret_escapes(s: str) -> str:
    """Interpret escape sequences in a string."""
    result = []
    i = 0
    while i < len(s):
        if s[i] == '\\' and i + 1 < len(s):
            c = s[i + 1]
            if c == 'n':
                result.append('\n')
            elif c == 't':
                result.append('\t')
            elif c == 'r':
                result.append('\r')
            elif c == '\\':
                result.append('\\')
            elif c == '"':
                result.append('"')
            elif c == '0':
                # Octal: \0NNN
                oct_str = ""
                j = i + 2
                while j < len(s) and j < i + 5 and '0' <= s[j] <= '7':
                    oct_str += s[j]
                    j += 1
                if oct_str:
                    try:
                        result.append(chr(int(oct_str, 8)))
                    except (ValueError, OverflowError):
                        pass
                    i = j
                    continue
                else:
                    result.append('\0')
            else:
                result.append(c)
            i += 2
        else:
            result.append(s[i])
            i += 1
    return "".join(result)
