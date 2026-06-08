"""cp: copy files and directories."""

import os
import sys
import shutil


def run(args: list[str]) -> int:
    interactive = False
    force = False
    verbose = False
    no_clobber = False
    update = False
    recursive = False
    sources = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg in ("-i", "--interactive"):
            interactive = True
        elif arg in ("-f", "--force"):
            force = True
        elif arg in ("-v", "--verbose"):
            verbose = True
        elif arg in ("-n", "--no-clobber"):
            no_clobber = True
        elif arg in ("-u", "--update"):
            update = True
        elif arg in ("-r", "-R", "--recursive"):
            recursive = True
        elif arg.startswith("-") and len(arg) > 1:
            print(f"cp: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            sources.append(arg)
        i += 1

    if len(sources) < 2:
        print("cp: missing operand", file=sys.stderr)
        return 1

    dest = sources[-1]
    srcs = sources[:-1]

    dest_is_dir = os.path.isdir(dest)

    if len(srcs) > 1 and not dest_is_dir:
        print(f"cp: target '{dest}' is not a directory", file=sys.stderr)
        return 1

    if interactive and force:
        force = False

    exit_code = 0
    for src in srcs:
        if dest_is_dir:
            dest_path = os.path.join(dest, os.path.basename(src))
        else:
            dest_path = dest

        exit_code |= copy_one(
            src, dest_path,
            interactive, force, verbose, no_clobber, update, recursive,
        )

    return exit_code


def copy_one(
    src: str, dst: str,
    interactive: bool, force: bool, verbose: bool,
    no_clobber: bool, update: bool, recursive: bool,
) -> int:
    try:
        src_stat = os.lstat(src)
    except OSError as e:
        print(f"cp: cannot stat '{src}': {e}", file=sys.stderr)
        return 1

    if os.path.isdir(src):
        if not recursive:
            print(f"cp: -r not specified; omitting directory '{src}'", file=sys.stderr)
            return 1
        return copy_dir(
            src, dst, interactive, force, verbose, no_clobber, update,
        )

    # Update check
    if update and os.path.exists(dst):
        try:
            src_mtime = os.path.getmtime(src)
            dst_mtime = os.path.getmtime(dst)
            if src_mtime <= dst_mtime:
                if verbose:
                    print(f"skip '{dst}' (newer or equal)")
                return 0
        except OSError:
            pass

    # No-clobber check
    if no_clobber and os.path.exists(dst):
        if verbose:
            print(f"skip '{dst}' (no-clobber)")
        return 0

    # Interactive check
    if interactive and os.path.exists(dst):
        try:
            response = input(f"cp: overwrite '{dst}'? ")
            if response.lower() not in ("y", "yes"):
                return 0
        except (EOFError, OSError):
            pass

    # Force: remove destination first
    if force and os.path.exists(dst):
        try:
            if os.path.isdir(dst):
                shutil.rmtree(dst)
            else:
                os.remove(dst)
        except OSError:
            pass

    try:
        shutil.copy2(src, dst)
        if verbose:
            print(f"'{src}' -> '{dst}'")
        return 0
    except OSError as e:
        print(f"cp: cannot copy '{src}' to '{dst}': {e}", file=sys.stderr)
        return 1


def copy_dir(
    src: str, dst: str,
    interactive: bool, force: bool, verbose: bool,
    no_clobber: bool, update: bool,
) -> int:
    try:
        os.makedirs(dst, exist_ok=True)
    except OSError as e:
        print(f"cp: cannot create directory '{dst}': {e}", file=sys.stderr)
        return 1

    exit_code = 0
    try:
        for entry in os.listdir(src):
            src_path = os.path.join(src, entry)
            dst_path = os.path.join(dst, entry)

            if os.path.isdir(src_path):
                exit_code |= copy_dir(
                    src_path, dst_path, interactive, force, verbose, no_clobber, update,
                )
            else:
                exit_code |= copy_one(
                    src_path, dst_path, interactive, force, verbose, no_clobber, update, True,
                )
    except OSError as e:
        print(f"cp: cannot read directory '{src}': {e}", file=sys.stderr)
        return 1

    return exit_code
