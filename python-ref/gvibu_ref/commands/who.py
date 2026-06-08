import os
import struct
import sys
from datetime import datetime

# utmp struct format for Linux x86_64:
# short ut_type (2) + padding (2) + int ut_pid (4) + ut_line[32] + ut_id[4]
# + ut_user[32] + ut_host[256] + padding[8] + int ut_session (4)
# + padding[4] + time_t (8) + addr_v6[16] + reserved[20]
UTMP_FORMAT = "hhi32s4s32s256s8xi4x2i16s20x"
UTMP_SIZE = struct.calcsize(UTMP_FORMAT)

USER_PROCESS = 7
LOGIN_PROCESS = 6


def read_cstr(data: bytes) -> str:
    null = data.find(b"\x00")
    return data[:null].decode("utf-8", errors="replace") if null >= 0 else data.decode("utf-8", errors="replace")


def run(args: list[str]) -> int:
    for arg in args:
        if arg.startswith("-") and len(arg) > 1:
            print(f"who: invalid option: {arg}", file=sys.stderr)
            return 1

    utmp_paths = ["/var/run/utmp", "/run/utmp", "/var/log/wtmp"]
    path = None
    for p in utmp_paths:
        if os.path.exists(p):
            path = p
            break

    if path is None:
        print("who: cannot find utmp file", file=sys.stderr)
        return 1

    try:
        with open(path, "rb") as f:
            data = f.read()
    except OSError as e:
        print(f"who: {path}: {e}", file=sys.stderr)
        return 1

    found = False
    for offset in range(0, len(data) - UTMP_SIZE + 1, UTMP_SIZE):
        chunk = data[offset:offset + UTMP_SIZE]
        try:
            fields = struct.unpack(UTMP_FORMAT, chunk)
        except struct.error:
            continue

        ut_type = fields[0]
        if ut_type not in (USER_PROCESS, LOGIN_PROCESS):
            continue

        ut_user = read_cstr(fields[5])
        ut_line = read_cstr(fields[3])
        ut_host = read_cstr(fields[6])

        if not ut_user or ut_user == "LOGIN":
            continue

        tv_sec = fields[9]

        if tv_sec > 0:
            dt = datetime.fromtimestamp(tv_sec)
            print(f"{ut_user:<8} {ut_line:<12} {dt.strftime('%Y-%m-%d %H:%M:%S')} ({ut_host})")
        else:
            print(f"{ut_user:<8} {ut_line:<12}")

        found = True

    if not found:
        print("no users logged in")

    return 0
