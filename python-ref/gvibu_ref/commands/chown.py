"""chown: change file owner and group."""
import os
import pwd
import grp


def _lookup_user(name: str) -> int | None:
    try:
        return int(name)
    except ValueError:
        pass
    try:
        return pwd.getpwnam(name).pw_uid
    except KeyError:
        return None


def _lookup_group(name: str) -> int | None:
    try:
        return int(name)
    except ValueError:
        pass
    try:
        return grp.getgrnam(name).gr_gid
    except KeyError:
        return None


def _parse_owner(s: str) -> tuple[int | None, int | None]:
    if not s:
        return (None, None)
    if ':' in s:
        colon_idx = s.index(':')
        user_part = s[:colon_idx]
        group_part = s[colon_idx + 1:]
        uid = _lookup_user(user_part) if user_part else None
        gid = _lookup_group(group_part) if group_part else None
        return (uid, gid)
    else:
        uid = _lookup_user(s)
        return (uid, None)


def _chown_path(path: str, uid: int | None, gid: int | None, verbose: bool) -> int:
    try:
        os.chown(path, uid if uid is not None else -1, gid if gid is not None else -1)
        if verbose:
            print(f"changed ownership of '{path}'")
        return 0
    except OSError as e:
        print(f"chown: {path}: {e}", file=os.sys.stderr)
        return 1


def _chown_recursive(path: str, uid: int | None, gid: int | None, verbose: bool) -> int:
    exit_code = _chown_path(path, uid, gid, verbose)
    try:
        for entry in os.scandir(path):
            child_path = entry.path
            if entry.is_dir(follow_symlinks=False):
                exit_code |= _chown_recursive(child_path, uid, gid, verbose)
            else:
                exit_code |= _chown_path(child_path, uid, gid, verbose)
    except PermissionError:
        pass
    return exit_code


def run(args: list[str]) -> int:
    if not args:
        print("chown: missing operand", file=os.sys.stderr)
        return 1

    recursive = False
    verbose = False
    owner_arg = None
    files = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == '-R':
            recursive = True
        elif arg == '-v':
            verbose = True
        elif arg.startswith('-') and len(arg) > 1:
            print(f"chown: invalid option: {arg}", file=os.sys.stderr)
            return 1
        elif owner_arg is None:
            owner_arg = arg
        else:
            files.append(arg)
        i += 1

    if owner_arg is None:
        print("chown: missing operand", file=os.sys.stderr)
        return 1

    if not files:
        print(f"chown: missing operand after '{owner_arg}'", file=os.sys.stderr)
        return 1

    uid, gid = _parse_owner(owner_arg)
    if uid is None and gid is None:
        print(f"chown: invalid owner: '{owner_arg}'", file=os.sys.stderr)
        return 1

    exit_code = 0
    for fname in files:
        try:
            if recursive and os.path.isdir(fname):
                exit_code |= _chown_recursive(fname, uid, gid, verbose)
            else:
                exit_code |= _chown_path(fname, uid, gid, verbose)
        except OSError as e:
            print(f"chown: {fname}: {e}", file=os.sys.stderr)
            exit_code = 1

    return exit_code
