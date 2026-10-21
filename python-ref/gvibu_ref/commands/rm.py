import os
import sys
import shutil
import stat


def run(args: list[str]) -> int:
    recursive = False
    force = False
    verbose = False
    dir_only = False
    interactive = False
    preserve_root = True
    targets = []
    parsing_options = True

    i = 0
    while i < len(args):
        arg = args[i]
        if parsing_options and arg == "--":
            parsing_options = False
        elif parsing_options and arg in ("-r", "-R", "--recursive"):
            recursive = True
        elif parsing_options and arg in ("-f", "--force"):
            force = True
        elif parsing_options and arg in ("-v", "--verbose"):
            verbose = True
        elif parsing_options and arg in ("-d", "--dir"):
            dir_only = True
        elif parsing_options and arg in ("-i", "--interactive"):
            interactive = True
        elif parsing_options and arg == "--preserve-root":
            preserve_root = True
        elif parsing_options and arg == "--no-preserve-root":
            preserve_root = False
        elif parsing_options and arg in ("-rf", "-fr", "-Rf", "-fR"):
            recursive = True
            force = True
        elif parsing_options and arg.startswith("-") and len(arg) > 1:
            print(f"rm: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            targets.append(arg)
        i += 1

    if not targets:
        print("rm: missing operand", file=sys.stderr)
        return 1

    exit_code = 0

    if preserve_root:
        for target in targets:
            try:
                if os.path.realpath(target) == os.path.sep:
                    print("rm: it is dangerous to operate recursively on '/'", file=sys.stderr)
                    print("rm: use --no-preserve-root to override this failsafe", file=sys.stderr)
                    return 1
            except OSError:
                pass

    for target in targets:
        try:
            if interactive and not force:
                answer = input(f"rm: remove '{target}'? ")
                if answer.strip().lower() not in ("y", "yes"):
                    continue
            if dir_only and stat.S_ISDIR(os.lstat(target).st_mode):
                os.rmdir(target)
            elif recursive:
                if os.path.isdir(target):
                    shutil.rmtree(target)
                else:
                    os.remove(target)
            else:
                os.remove(target)
            if verbose:
                print(f"removed '{target}'")
        except OSError as e:
            if not force:
                print(f"rm: cannot remove '{target}': {e}", file=sys.stderr)
                exit_code = 1

    return exit_code
