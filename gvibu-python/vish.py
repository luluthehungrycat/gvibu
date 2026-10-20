#!/usr/bin/env python3
"""Small Python REPL and command dispatcher for GVIBU."""

import importlib
import os
import pkgutil
import sys
import traceback

# Make gvibu_ref importable regardless of the current working directory.
_THIS_DIR = os.path.dirname(os.path.abspath(__file__))
_PYTHON_REF_DIR = os.path.normpath(os.path.join(_THIS_DIR, "..", "python-ref"))
if _PYTHON_REF_DIR not in sys.path:
    sys.path.insert(0, _PYTHON_REF_DIR)

HELP_TEXT = (
    "GVIBU Python shell (VISH) — universal coreutils\n"
    "Built-in commands: help, version, commands, exit, quit\n"
    "Type a command name to run it, or one of the built-ins above.\n"
)
VERSION_FALLBACK = "0.1.0"
COMMANDS = {}

# These Python module names differ from their public command names.
_COMMAND_ALIASES = {
    "env_cmd": ("env",),
    "test_cmd": ("test", "["),
    "which_cmd": ("which",),
}


def _discover_command_package():
    """Return the importable package name that exposes GVIBU commands."""
    try:
        import gvibu_ref.commands  # noqa: F401

        return "gvibu_ref.commands"
    except Exception:
        return "commands"


def _load_commands():
    """Load every command module, including modules omitted from __all__."""
    global COMMANDS
    COMMANDS = {}
    pkg_name = _discover_command_package()
    try:
        pkg = importlib.import_module(pkg_name)
    except Exception:
        return COMMANDS

    # __all__ is intentionally a small convenience export list in gvibu_ref;
    # discover modules from the package so the REPL exposes the full CLI set.
    module_names = sorted(
        module.name for module in pkgutil.iter_modules(getattr(pkg, "__path__", []))
    )
    for mod_name in module_names:
        if mod_name.startswith("_") or mod_name == "__init__":
            continue
        try:
            mod = importlib.import_module(f"{pkg_name}.{mod_name}")
        except Exception:
            continue
        run_fn = getattr(mod, "run", None)
        if not callable(run_fn):
            continue
        public_names = _COMMAND_ALIASES.get(mod_name, (mod_name,))
        for public_name in public_names:
            COMMANDS[public_name] = mod
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
    for name in sorted(COMMANDS):
        sys.stdout.write(f"{name}\n")
    return 0


_EXIT_REPL = object()

_BUILTINS = {
    "help": _builtin_help,
    "version": _builtin_version,
    "commands": _builtin_commands,
    "exit": lambda: _EXIT_REPL,
    "quit": lambda: _EXIT_REPL,
}


def _run_dispatch(name, args):
    """Run a GVIBU command and return its exit code."""
    if name in COMMANDS:
        try:
            command_args = ["[", *args] if name == "[" else args
            return int(COMMANDS[name].run(command_args))
        except SystemExit as exc:
            return int(exc.code) if exc.code is not None else 0
        except Exception:
            traceback.print_exc()
            return 1
    sys.stderr.write(f"{name}: command not found\n")
    return 1


def _execute_line(line):
    """Parse and run one command line."""
    parts = line.strip().split()
    if not parts:
        return None
    name, args = parts[0], parts[1:]
    if name in _BUILTINS:
        return _BUILTINS[name]()
    return _run_dispatch(name, args)


def run_repl():
    """Run an input loop; EOF exits cleanly and Ctrl-C reprompts."""
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
        if _execute_line(line) is _EXIT_REPL:
            return 0


def run_subcommand(argv):
    """Run a command, or start the REPL when argv is empty."""
    if not argv:
        return run_repl()
    name, args = argv[0], argv[1:]
    if name in _BUILTINS:
        result = _BUILTINS[name]()
        return 0 if result is _EXIT_REPL else int(result)
    return _run_dispatch(name, args)


def main():
    _load_commands()
    sys.exit(run_subcommand(sys.argv[1:]))


if __name__ == "__main__":
    main()
