import sys


def run(args: list[str]) -> int:
    if args:
        print("uptime: too many arguments", file=sys.stderr)
        return 1

    try:
        with open("/proc/uptime") as f:
            seconds = float(f.read().split()[0])
    except (FileNotFoundError, IndexError, ValueError):
        seconds = 0.0

    seconds = int(seconds)
    days = seconds // 86400
    hours = (seconds % 86400) // 3600
    minutes = (seconds % 3600) // 60

    if days > 0:
        print(f"up {days} day{'s' if days != 1 else ''}, {hours} hour{'s' if hours != 1 else ''}, {minutes} minute{'s' if minutes != 1 else ''}")
    elif hours > 0:
        print(f"up {hours} hour{'s' if hours != 1 else ''}, {minutes} minute{'s' if minutes != 1 else ''}")
    else:
        m = minutes if minutes > 0 else 1
        print(f"up {m} minute{'s' if m != 1 else ''}")
    return 0
