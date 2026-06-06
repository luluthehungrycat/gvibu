#!/usr/bin/env python3
"""gvibu-ref: Unix-style multicall utility (Python reference)."""

import sys
import os


sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
COMMANDS = {
    "true": None,
    "false": None,
    "echo": None,
    "pwd": None,
    "basename": None,
    "dirname": None,
    "cat": None,
    "wc": None,
    "head": None,
    "yes": None,
    "printenv": None,
    "sleep": None,
}


def get_command_name() -> str:
    """Determine command name from argv[0] or first argument."""
    argv0 = os.path.basename(sys.argv[0])
    if argv0 in COMMANDS:
        return argv0
    if len(sys.argv) > 1 and sys.argv[1] in COMMANDS:
        return sys.argv[1]
    return argv0


def dispatch():
    """Dispatch to the appropriate command."""
    try:
        from gvibu_ref.commands import true, false, echo, pwd, basename, dirname, cat, wc, head, yes, printenv, sleep
    except ModuleNotFoundError:
        from commands import true, false, echo, pwd, basename, dirname, cat, wc, head, yes, printenv, sleep

    COMMANDS["true"] = true
    COMMANDS["false"] = false
    COMMANDS["echo"] = echo
    COMMANDS["pwd"] = pwd
    COMMANDS["basename"] = basename
    COMMANDS["dirname"] = dirname
    COMMANDS["cat"] = cat
    COMMANDS["wc"] = wc
    COMMANDS["head"] = head
    COMMANDS["yes"] = yes
    COMMANDS["printenv"] = printenv
    COMMANDS["sleep"] = sleep

    cmd_name = get_command_name()
    argv0 = os.path.basename(sys.argv[0])

    if cmd_name in COMMANDS:
        if argv0 == cmd_name:
            args = sys.argv[1:]
        elif len(sys.argv) > 1 and sys.argv[1] == cmd_name:
            args = sys.argv[2:]
        else:
            args = sys.argv[1:]

        try:
            result = COMMANDS[cmd_name].run(args)
            sys.exit(result)
        except Exception as e:
            print(f"{cmd_name}: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        print(f"gvibu-ref: {cmd_name}: command not found", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    dispatch()
