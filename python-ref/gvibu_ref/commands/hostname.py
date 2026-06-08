import sys


def run(args: list[str]) -> int:
    if args:
        print("hostname: too many arguments", file=sys.stderr)
        return 1

    try:
        with open("/etc/hostname") as f:
            hostname = f.read().strip()
    except FileNotFoundError:
        try:
            with open("/proc/sys/kernel/hostname") as f:
                hostname = f.read().strip()
        except FileNotFoundError:
            hostname = ""

    if not hostname:
        print("hostname: cannot determine hostname", file=sys.stderr)
        return 1

    print(hostname)
    return 0
