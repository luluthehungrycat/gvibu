import os
import sys
import pwd
import grp


def uid_to_name(uid: int) -> str:
    try:
        return pwd.getpwuid(uid).pw_name
    except KeyError:
        return str(uid)


def gid_to_name(gid: int) -> str:
    try:
        return grp.getgrgid(gid).gr_name
    except KeyError:
        return str(gid)


def run(args: list[str]) -> int:
    flag_u = False
    flag_g = False
    flag_groups = False
    flag_name = False
    flag_real = False

    for arg in args:
        for ch in arg[1:]:
            if ch == "u":
                flag_u = True
            elif ch == "g":
                flag_g = True
            elif ch == "G":
                flag_groups = True
            elif ch == "n":
                flag_name = True
            elif ch == "r":
                flag_real = True
            else:
                print(f"id: invalid option: -{ch}", file=sys.stderr)
                return 1

    if flag_name and not flag_u and not flag_g and not flag_groups:
        print("id: cannot print only names in default format", file=sys.stderr)
        return 1

    if not flag_u and not flag_g and not flag_groups:
        euid = os.geteuid()
        egid = os.getegid()
        uname = uid_to_name(euid)
        gname = gid_to_name(egid)
        groups = os.getgroups()

        group_strs = [f"{g}({gid_to_name(g)})" for g in groups]
        print(f"uid={euid}({uname}) gid={egid}({gname}) groups={','.join(group_strs)}")
        return 0

    if flag_u:
        uid = os.getuid() if flag_real else os.geteuid()
        if flag_name:
            print(uid_to_name(uid))
        else:
            print(uid)

    if flag_g:
        gid = os.getgid() if flag_real else os.getegid()
        if flag_name:
            print(gid_to_name(gid))
        else:
            print(gid)

    if flag_groups:
        groups = os.getgroups()
        if flag_name:
            print(" ".join(gid_to_name(g) for g in groups))
        else:
            print(" ".join(str(g) for g in groups))

    return 0
