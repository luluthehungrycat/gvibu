import os
import sys
import signal


SIGNAL_MAP = {v: k for k, v in vars(signal).items() if k.startswith("SIG") and not k.startswith("SIG_")}


def parse_signal(s: str) -> int | None:
    if s.isdigit():
        n = int(s)
        if 1 <= n <= 31:
            return n
    upper = s.upper()
    sig_num = getattr(signal, f"SIG{upper}", None)
    return sig_num if sig_num is not None else None


def run(args: list[str]) -> int:
    if not args:
        print("kill: missing operand", file=sys.stderr)
        return 1

    if len(args) == 1 and args[0] == "-l":
        for sig_num in sorted(SIGNAL_MAP):
            name = SIGNAL_MAP[sig_num].replace("SIG", "")
            print(f"{sig_num:>2}) {name}")
        return 0

    if args[0] == "-s":
        if len(args) < 3:
            print("kill: missing operand", file=sys.stderr)
            return 1
        sig = parse_signal(args[1])
        if sig is None:
            print(f"kill: invalid signal: {args[1]}", file=sys.stderr)
            return 1
        try:
            pid = int(args[2])
        except ValueError:
            print(f"kill: invalid pid: {args[2]}", file=sys.stderr)
            return 1
    elif args[0].startswith("-") and len(args[0]) > 1:
        sig_str = args[0][1:]
        if len(args) < 2:
            print("kill: missing operand", file=sys.stderr)
            return 1
        sig = parse_signal(sig_str)
        if sig is None:
            print(f"kill: invalid signal: {sig_str}", file=sys.stderr)
            return 1
        try:
            pid = int(args[1])
        except ValueError:
            print(f"kill: invalid pid: {args[1]}", file=sys.stderr)
            return 1
    else:
        try:
            pid = int(args[0])
        except ValueError:
            print(f"kill: invalid pid: {args[0]}", file=sys.stderr)
            return 1
        sig = signal.SIGTERM

    try:
        os.kill(pid, sig)
        return 0
    except OSError as e:
        print(f"kill: ({pid}) - {e}", file=sys.stderr)
        return 1
