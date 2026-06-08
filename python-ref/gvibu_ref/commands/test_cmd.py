"""test / [: evaluate expression."""
import os
import stat
import sys


def _file_exists(path: str) -> bool:
    try:
        os.stat(path)
        return True
    except OSError:
        return False


def _is_file(path: str) -> bool:
    try:
        return stat.S_ISREG(os.stat(path).st_mode)
    except OSError:
        return False


def _is_dir(path: str) -> bool:
    try:
        return stat.S_ISDIR(os.stat(path).st_mode)
    except OSError:
        return False


def _is_executable(path: str) -> bool:
    try:
        st = os.stat(path)
        return bool(st.st_mode & (stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH))
    except OSError:
        return False


def _is_writable(path: str) -> bool:
    return os.access(path, os.W_OK)


def _is_readable(path: str) -> bool:
    return os.access(path, os.R_OK)


def _is_nonempty(path: str) -> bool:
    try:
        return os.stat(path).st_size > 0
    except OSError:
        return False


def _eval_unary(op: str, val: str) -> bool:
    if op == '-n':
        return bool(val)
    elif op == '-z':
        return not val
    elif op == '-f':
        return _is_file(val)
    elif op == '-d':
        return _is_dir(val)
    elif op == '-e':
        return _file_exists(val)
    elif op == '-x':
        return _is_executable(val)
    elif op == '-w':
        return _is_writable(val)
    elif op == '-r':
        return _is_readable(val)
    elif op == '-s':
        return _is_nonempty(val)
    return False


def _parse_num(s: str) -> int | None:
    s = s.strip()
    if not s:
        return None
    try:
        return int(s)
    except ValueError:
        return None


def _eval_binary(left: str, op: str, right: str) -> bool:
    if op == '=':
        return left == right
    elif op == '!=':
        return left != right
    elif op == '-eq':
        a, b = _parse_num(left), _parse_num(right)
        return a is not None and b is not None and a == b
    elif op == '-ne':
        a, b = _parse_num(left), _parse_num(right)
        return a is not None and b is not None and a != b
    elif op == '-lt':
        a, b = _parse_num(left), _parse_num(right)
        return a is not None and b is not None and a < b
    elif op == '-le':
        a, b = _parse_num(left), _parse_num(right)
        return a is not None and b is not None and a <= b
    elif op == '-gt':
        a, b = _parse_num(left), _parse_num(right)
        return a is not None and b is not None and a > b
    elif op == '-ge':
        a, b = _parse_num(left), _parse_num(right)
        return a is not None and b is not None and a >= b
    return False


def run(args: list[str]) -> int:
    if not args:
        print("test: missing operand", file=sys.stderr)
        return 1

    test_args: list[str]
    if args[0] == '[':
        if len(args) < 2 or args[-1] != ']':
            print("test: missing ']'", file=sys.stderr)
            return 1
        test_args = args[1:-1]
    else:
        test_args = args

    if not test_args:
        return 1

    # Simple recursive descent evaluator
    # Supported: unary ops, binary ops, ! prefix
    # For simplicity, no -a/-o (AND/OR) support in this version
    i = 0
    not_flag = False

    # Check for leading !
    if test_args[0] == '!':
        not_flag = True
        i = 1

    if i >= len(test_args):
        return 1

    result = False

    # Check for unary op
    if test_args[i] in ('-n', '-z', '-f', '-d', '-e', '-x', '-w', '-r', '-s'):
        if i + 1 >= len(test_args):
            print(f"test: missing argument after '{test_args[i]}'", file=sys.stderr)
            return 1
        result = _eval_unary(test_args[i], test_args[i + 1])
        i += 2
    elif i + 2 < len(test_args) and test_args[i + 1] in ('=', '!=', '-eq', '-ne', '-lt', '-le', '-gt', '-ge'):
        result = _eval_binary(test_args[i], test_args[i + 1], test_args[i + 2])
        i += 3
    else:
        # Plain string — test if non-empty
        result = bool(test_args[i])
        i += 1

    if not_flag:
        result = not result

    return 0 if result else 1
