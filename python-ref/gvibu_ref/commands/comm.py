"""comm: compare two sorted files line by line."""
import sys


def run(args: list[str]) -> int:
    flag1 = True
    flag2 = True
    flag3 = True

    files: list[str] = []
    for arg in args:
        if arg == "-1":
            flag1 = False
        elif arg == "-2":
            flag2 = False
        elif arg == "-3":
            flag3 = False
        elif arg.startswith('-') and len(arg) > 1:
            print(f"comm: invalid option: {arg}", file=sys.stderr)
            return 1
        else:
            files.append(arg)

    if len(files) < 2:
        print("comm: missing operand", file=sys.stderr)
        return 1
    if len(files) > 2:
        print(f"comm: extra operand: {files[2]}", file=sys.stderr)
        return 1

    try:
        fh1 = sys.stdin if files[0] == "-" else open(files[0])
        fh2 = sys.stdin if files[1] == "-" else open(files[1])
    except OSError as e:
        print(f"comm: {e}", file=sys.stderr)
        return 1

    line1 = None
    line2 = None

    with fh1 if files[0] != "-" else _noop(fh1) as f1, \
         fh2 if files[1] != "-" else _noop(fh2) as f2:
        while True:
            if line1 is None:
                line1 = f1.readline()
            if line2 is None:
                line2 = f2.readline()

            if not line1 and not line2:
                break

            if not line1:
                if flag2:
                    print(f"\t{line2.rstrip()}")
                line2 = None
                for l in f2:
                    if flag2:
                        print(f"\t{l.rstrip()}")
                break

            if not line2:
                if flag1:
                    print(f"{line1.rstrip()}")
                line1 = None
                for l in f1:
                    if flag1:
                        print(f"{l.rstrip()}")
                break

            l1 = line1.rstrip('\n\r')
            l2 = line2.rstrip('\n\r')

            if l1 < l2:
                if flag1:
                    print(l1)
                line1 = None
                line2 = line2  # keep
            elif l1 > l2:
                if flag2:
                    print(f"\t{l2}")
                line2 = None
                line1 = line1  # keep
            else:
                if flag3:
                    print(f"\t\t{l1}")
                line1 = None
                line2 = None

    return 0


class _noop:
    """Context manager for stdin (no-op close)."""
    def __init__(self, fh):
        self._fh = fh
    def __enter__(self):
        return self._fh
    def __exit__(self, *args):
        pass
