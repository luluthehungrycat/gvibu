import os
import sys


def run(args: list[str]) -> int:
    """Execute pwd command.

    Args:
        args: Command-line arguments after the command name.
    Returns:
        Exit code as int.
    """
    # No options/operands supported. If any arguments are provided, return usage error.
    if len(args) > 0:
        print("pwd: usage: pwd", file=sys.stderr)
        return 2
    try:
        cwd = os.getcwd()
    except Exception as e:
        print(f"pwd: {e}", file=sys.stderr)
        return 1
    print(cwd)
    return 0
