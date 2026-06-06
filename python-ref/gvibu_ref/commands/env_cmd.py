"""env: print or set environment variables."""

import os
import subprocess  # nosec
import sys


def _print_env() -> None:
    for key, value in sorted(os.environ.items()):
        print(f"{key}={value}")


def run(args: list[str]) -> int:
    ignore_env = False
    unset_vars: list[str] = []
    cmd_and_args: list[str] = []
    set_vars: dict[str, str] = {}
    i = 0

    while i < len(args):
        arg = args[i]
        if arg == "-i" or arg == "--ignore-environment":
            ignore_env = True
        elif arg == "-u" or arg == "--unset":
            i += 1
            if i >= len(args):
                print("env: option requires an argument: -u", file=sys.stderr)
                return 1
            unset_vars.append(args[i])
        else:
            break
        i += 1

    # Collect VAR=VAL assignments and [COMMAND [ARG]...]
    assignments_done = False
    for arg in args[i:]:
        if not assignments_done and "=" in arg and not arg.startswith("-"):
            key, _, val = arg.partition("=")
            set_vars[key] = val
        else:
            assignments_done = True
            cmd_and_args.append(arg)

    # Build environment
    if ignore_env:
        env = {}
    else:
        env = dict(os.environ)

    for key, val in set_vars.items():
        env[key] = val
    for key in unset_vars:
        env.pop(key, None)

    if not cmd_and_args:
        for key, value in sorted(env.items()):
            print(f"{key}={value}")
        return 0

    # Run command
    try:
        result = subprocess.run(
            cmd_and_args,
            env=env,
        )
        return result.returncode
    except FileNotFoundError:
        print(f"env: {cmd_and_args[0]}: No such file or directory", file=sys.stderr)
        return 127
    except PermissionError:
        print(f"env: {cmd_and_args[0]}: Permission denied", file=sys.stderr)
        return 126
    except OSError as e:
        print(f"env: {e}", file=sys.stderr)
        return 1
