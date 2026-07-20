#!/usr/bin/env python3
"""vish (Python) — GVIBU REPL.

Architecture:
  - Imports + path setup  -> makes ``gvibu_ref`` importable from CWD-independent locations
  - Constants             -> HELP_TEXT, VERSION_FALLBACK
  - _load_commands()      -> uses importlib to populate COMMANDS dict
  - get_version()         -> gvibu_ref.__version__ with fallback to "0.1.0"
  - _builtin_help/_version/_commands  -> built-in command handlers
  - _run_dispatch()       -> calls module.run(args), returns int exit code
  - _execute_line()       -> REPL line parser / dispatcher
  - run_repl()            -> input() loop, EOFError -> clean exit,
                             KeyboardInterrupt -> ^C + re-prompt
  - run_subcommand()      -> subcommand-mode entry; no-args falls through to REPL
  - main()                -> entry point; argv[1:] -> run_subcommand
"""
import importlib
import importlib.util
import os
import sys
import traceback

# Make gvibu_ref importable regardless of CWD.
_THIS_DIR = os.path.dirname(os.path.abspath(__file__))
_PYTHON_REF_DIR = os.path.normpath(os.path.join(_THIS_DIR, "..", "python-ref"))
if _PYTHON_REF_DIR not in sys.path:
    sys.path.insert(0, _PYTHON_REF_DIR)
_FALLBACK_GVIBU_PY = os.path.join(_THIS_DIR, "gvibu.py")

HELP_TEXT = (
    "gvibu \u2014 universal coreutils\n"
    "Built-in commands: help, version, commands, exit, quit\n"
    "Type a command name to run it, or one of the built-ins above.\n"
)

VERSION_FALLBACK = "0.1.0"

COMMANDS = {}


def _discover_command_package():
    """Return the importable package name that exposes ``COMMANDS``."""
    try:
        import gvibu_ref.commands  # noqa: F401

        return "gvibu_ref.commands"
    except Exception:
        return "commands"


def _load_commands():
    """Populate the COMMANDS dict by importing each command module."""
    global COMMANDS
    COMMANDS = {}
    pkg_name = _discover_command_package()
    try:
        pkg = importlib.import_module(pkg_name)
    except Exception:
        return COMMANDS

    for mod_name in sorted(getattr(pkg, "__all__", []) or dir(pkg)):
        if not mod_name or mod_name.startswith("_"):
            continue
        try:
            mod = importlib.import_module(f"{pkg_name}.{mod_name}")
        except Exception:
            continue
        run_fn = getattr(mod, "run", None)
        if callable(run_fn):
            COMMANDS[mod_name] = mod
    return COMMANDS


def get_version():
    try:
        import gvibu_ref

        return getattr(gvibu_ref, "__version__", VERSION_FALLBACK)
    except Exception:
        return VERSION_FALLBACK


def _builtin_help():
    sys.stdout.write(HELP_TEXT)
    return 0


def _builtin_version():
    sys.stdout.write(f"gvibu v{get_version()}\n")
    return 0


def _builtin_commands():
    for name in sorted(COMMANDS.keys()):
        sys.stdout.write(f"{name}\n")
    return 0


_BUILTINS = {
    "help": _builtin_help,
    "version": _builtin_version,
    "commands": _builtin_commands,
    "exit": lambda: (_ for _ in ()).throw(SystemExit(0)),
    "quit": lambda: (_ for _ in ()).throw(SystemExit(0)),
}


def _run_dispatch(name, args):
    """Run a real gvibu command, returning its exit code."""
    if name in COMMANDS:
        try:
            return int(COMMANDS[name].run(args))
        except SystemExit as e:
            return int(e.code) if e.code is not None else 0
        except Exception:
            traceback.print_exc()
            return 1
    sys.stderr.write(f"{name}: command not found\n")
    return 1


def _execute_line(line):
    """Parse a single REPL line and dispatch it. Return exit code or None to continue."""
    trimmed = line.strip()
    if not trimmed:
        return None
    parts = trimmed.split()
    name, args = parts[0], parts[1:]
    if name in _BUILTINS:
        try:
            return _BUILTINS[name]()
        except SystemExit as e:
            return int(e.code) if e.code is not None else 0
    return _run_dispatch(name, args)


def run_repl():
    """Read-eval-print loop. EOFError -> clean exit, KeyboardInterrupt -> ^C + re-prompt."""
    while True:
        try:
            if sys.stdin.isatty():
                sys.stdout.write("gvibu> ")
                sys.stdout.flush()
            line = input()
        except EOFError:
            if sys.stdin.isatty():
                sys.stdout.write("\n")
            return 0
        except KeyboardInterrupt:
            sys.stdout.write("^C\n")
            sys.stdout.flush()
            continue
        _execute_line(line)


def run_subcommand(argv):
    """Subcommand mode entry. Empty argv -> run REPL."""
    if not argv:
        return run_repl()
    name, args = argv[0], argv[1:]
    if name in _BUILTINS:
        try:
            return _BUILTINS[name]()
        except SystemExit as e:
            return int(e.code) if e.code is not None else 0
    return _run_dispatch(name, args)


def main():
    _load_commands()
    sys.exit(run_subcommand(sys.argv[1:]))


if __name__ == "__main__":
    main()
