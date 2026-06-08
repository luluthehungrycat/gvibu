"""chmod: change file mode bits."""
import os
import stat


def _parse_octal(s: str) -> int | None:
    if not s or len(s) > 4:
        return None
    for ch in s:
        if not ('0' <= ch <= '7'):
            return None
    try:
        return int(s, 8)
    except ValueError:
        return None


def _parse_symbolic(s: str) -> tuple[str, str, int] | None:
    """Parse symbolic mode like u+x, go-w, a=r"""
    if len(s) < 2:
        return None
    who_chars = []
    i = 0
    while i < len(s) and s[i] in 'ugoa':
        who_chars.append(s[i])
        i += 1
    if not who_chars or i >= len(s):
        return None
    op = s[i]
    if op not in '+-=':
        return None
    i += 1
    perm_str = s[i:]
    if not perm_str:
        return None
    perm_bits = 0
    for ch in perm_str:
        if ch == 'r':
            perm_bits |= 0b100
        elif ch == 'w':
            perm_bits |= 0b010
        elif ch == 'x':
            perm_bits |= 0b001
        else:
            return None
    # Expand who
    who = ''.join(who_chars)
    expanded = 0
    for w in who:
        if w == 'u':
            expanded |= perm_bits << 6
        elif w == 'g':
            expanded |= perm_bits << 3
        elif w == 'o':
            expanded |= perm_bits
        elif w == 'a':
            expanded |= (perm_bits << 6) | (perm_bits << 3) | perm_bits
    return (who, op, expanded)


def _apply_mode_change(current: int, who: str, op: str, bits: int) -> int:
    who_mask = 0
    for w in who:
        if w == 'u':
            who_mask |= 0o700
        elif w == 'g':
            who_mask |= 0o070
        elif w == 'o':
            who_mask |= 0o007
        elif w == 'a':
            who_mask |= 0o777
    if op == '+':
        return current | (bits & who_mask)
    elif op == '-':
        return current & ~(bits & who_mask)
    elif op == '=':
        return (current & ~who_mask) | (bits & who_mask)
    return current


def _chmod_path(path: str, mode: int, verbose: bool) -> int:
    try:
        os.chmod(path, mode)
        if verbose:
            print(f"mode of '{path}' changed")
        return 0
    except OSError as e:
        print(f"chmod: {path}: {e}", file=os.sys.stderr)
        return 1


def _chmod_recursive(path: str, mode: int, verbose: bool) -> int:
    exit_code = _chmod_path(path, mode, verbose)
    try:
        for entry in os.scandir(path):
            child_path = entry.path
            if entry.is_dir(follow_symlinks=False):
                exit_code |= _chmod_recursive(child_path, mode, verbose)
            else:
                exit_code |= _chmod_path(child_path, mode, verbose)
    except PermissionError:
        pass
    return exit_code


def run(args: list[str]) -> int:
    if not args:
        print("chmod: missing operand", file=os.sys.stderr)
        return 1

    recursive = False
    verbose = False
    mode_arg = None
    files = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == '-R':
            recursive = True
        elif arg == '-v':
            verbose = True
        elif arg.startswith('-') and len(arg) > 1:
            print(f"chmod: invalid option: {arg}", file=os.sys.stderr)
            return 1
        elif mode_arg is None:
            mode_arg = arg
        else:
            files.append(arg)
        i += 1

    if mode_arg is None:
        print("chmod: missing operand", file=os.sys.stderr)
        return 1

    mode_val = _parse_octal(mode_arg)
    if mode_val is None:
        sym = _parse_symbolic(mode_arg)
        if sym is not None:
            who, op, bits = sym
            current = 0o644  # default
            mode_val = _apply_mode_change(current, who, op, bits)
        else:
            print(f"chmod: invalid mode: '{mode_arg}'", file=os.sys.stderr)
            return 1

    if not files:
        print(f"chmod: missing operand after '{mode_arg}'", file=os.sys.stderr)
        return 1

    exit_code = 0
    for fname in files:
        try:
            if recursive and os.path.isdir(fname):
                exit_code |= _chmod_recursive(fname, mode_val, verbose)
            else:
                exit_code |= _chmod_path(fname, mode_val, verbose)
        except OSError as e:
            print(f"chmod: {fname}: {e}", file=os.sys.stderr)
            exit_code = 1

    return exit_code
