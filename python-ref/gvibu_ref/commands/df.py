"""df: report file system space usage."""

import os
import sys


def run(args: list[str]) -> int:
    human = False
    show_type = False
    explicit_paths = []

    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-h":
            human = True
        elif arg == "-T":
            show_type = True
        elif arg.startswith("-") and len(arg) > 1:
            for c in arg[1:]:
                if c == "h":
                    human = True
                elif c == "T":
                    show_type = True
                else:
                    print(f"df: invalid option: -{c}", file=sys.stderr)
                    return 1
        else:
            explicit_paths.append(arg)
        i += 1

    try:
        mounts = _read_mounts()
    except OSError as e:
        print(f"df: {e}", file=sys.stderr)
        return 1

    # Print header
    if show_type:
        print(f"{'Filesystem':<14} {'Type':<11} {'1K-blocks':>8} {'Used':>8} {'Available':>8} {'Use%':>3} Mounted on")
    else:
        print(f"{'Filesystem':<14} {'1K-blocks':>8} {'Used':>8} {'Available':>8} {'Use%':>3} Mounted on")

    exit_code = 0

    # Filter out pseudo filesystems
    pseudo_fs = {
        "rootfs", "proc", "sysfs", "cgroup", "devpts", "devtmpfs",
        "tmpfs", "pstore", "securityfs", "hugetlbfs", "mqueue",
        "debugfs", "tracefs", "configfs", "efivarfs", "fusectl",
    }

    for fs_file, mount_point, fs_type in mounts:
        if explicit_paths:
            if not any(mount_point == p or fs_file.startswith(p) for p in explicit_paths):
                continue

        if fs_type in pseudo_fs and not explicit_paths:
            continue

        try:
            st = os.statvfs(mount_point)
        except OSError:
            # A mount can disappear or be inaccessible while /proc/mounts
            # is being read; skip it like df does for unavailable mounts.
            continue

        frsize = st.f_frsize
        total = (st.f_blocks * frsize) // 1024
        avail = (st.f_bavail * frsize) // 1024
        used = total - (st.f_bfree * frsize) // 1024
        use_pct = (used * 100) // total if total > 0 else 0

        if human:
            total_h = _human_size(total * 1024)
            used_h = _human_size(used * 1024)
            avail_h = _human_size(avail * 1024)
            if show_type:
                print(f"{fs_file:<14} {fs_type:<11} {total_h:>5} {used_h:>8} {avail_h:>8} {use_pct:>3}% {mount_point}")
            else:
                print(f"{fs_file:<14} {total_h:>5} {used_h:>8} {avail_h:>8} {use_pct:>3}% {mount_point}")
        else:
            if show_type:
                print(f"{fs_file:<14} {fs_type:<11} {total:>8} {used:>8} {avail:>8} {use_pct:>3}% {mount_point}")
            else:
                print(f"{fs_file:<14} {total:>8} {used:>8} {avail:>8} {use_pct:>3}% {mount_point}")

    return exit_code


def _read_mounts() -> list[tuple[str, str, str]]:
    mounts = []
    with open("/proc/mounts") as f:
        for line in f:
            parts = line.split()
            if len(parts) >= 3:
                mounts.append((parts[0], parts[1], parts[2]))
    return mounts


def _human_size(bytes_val: int) -> str:
    units = ["", "K", "M", "G", "T", "P"]
    size = float(bytes_val)
    unit_idx = 0

    while size >= 1024.0 and unit_idx < len(units) - 1:
        size /= 1024.0
        unit_idx += 1

    if unit_idx == 0:
        return str(bytes_val)
    elif size >= 100.0:
        return f"{size:.0f}{units[unit_idx]}"
    elif size >= 10.0:
        return f"{size:.1f}{units[unit_idx]}"
    else:
        return f"{size:.1f}{units[unit_idx]}"
