#!/usr/bin/env python3
"""gvibu-ref: Unix-style multicall utility (Python reference)."""

import sys
import os


sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
COMMANDS = {
    "true": None,
    "false": None,
    "echo": None,
    "pwd": None,
    "basename": None,
    "dirname": None,
    "cat": None,
    "wc": None,
    "head": None,
    "yes": None,
    "printenv": None,
    "sleep": None,
    "touch": None,
    "seq": None,
    "which": None,
    "uname": None,
    "env": None,
    "whoami": None,
    "link": None,
    "unlink": None,
    "tee": None,
    "mkdir": None,
    "rmdir": None,
    "hostname": None,
    "logname": None,
    "readlink": None,
    "realpath": None,
    "uniq": None,
    "uptime": None,
    "id": None,
    "who": None,
    "kill": None,
    "cut": None,
    "tr": None,
    "mv": None,
    "rm": None,
    "ln": None,
    "chmod": None,
    "chown": None,
    "sort": None,
    "grep": None,
    "ls": None,
    "cp": None,
    "printf": None,
    "date": None,
    "expr": None,
    "split": None,
    "tail": None,
    "tac": None,
    "fold": None,
    "comm": None,
    "join": None,
    "nl": None,
    "shuf": None,
    "sum": None,
    "du": None,
    "df": None,
    "test": None,
    "[": None,
}


def get_command_name() -> str:
    """Determine command name from argv[0] or first argument."""
    argv0 = os.path.basename(sys.argv[0])
    if argv0 in COMMANDS:
        return argv0
    if len(sys.argv) > 1 and sys.argv[1] in COMMANDS:
        return sys.argv[1]
    return argv0


def dispatch():
    """Dispatch to the appropriate command."""
    try:
        from gvibu_ref.commands import (
            true, false, echo, pwd, basename, dirname, cat, wc, head,
            yes, printenv, sleep, touch, seq, which_cmd, uname, env_cmd, whoami,
            link, unlink, tee, mkdir, rmdir,
            hostname, logname, readlink, realpath, uniq, uptime,
            id, who, kill, cut, tr, mv, rm, ln,
            chmod, chown, sort, test_cmd,
            grep, ls, cp, printf, date, expr, split,
            tail, tac, fold, comm, expand, rev,
            join, nl, shuf, sum,
            du, df,
        )
    except ModuleNotFoundError:
        from commands import (
            yes, printenv, sleep, touch, seq, which_cmd, uname, env_cmd, whoami,
            link, unlink, tee, mkdir, rmdir,
            hostname, logname, readlink, realpath, uniq, uptime,
            id, who, kill, cut, tr, mv, rm, ln,
            chmod, chown, sort, test_cmd,
            grep, ls, cp, printf, date, expr, split,
            tail, tac, fold, comm, expand, rev,
            join, nl, shuf, sum,
            du, df,
        )

    COMMANDS["true"] = true
    COMMANDS["false"] = false
    COMMANDS["echo"] = echo
    COMMANDS["pwd"] = pwd
    COMMANDS["basename"] = basename
    COMMANDS["dirname"] = dirname
    COMMANDS["cat"] = cat
    COMMANDS["wc"] = wc
    COMMANDS["head"] = head
    COMMANDS["yes"] = yes
    COMMANDS["printenv"] = printenv
    COMMANDS["sleep"] = sleep
    COMMANDS["touch"] = touch
    COMMANDS["seq"] = seq
    COMMANDS["which"] = which_cmd
    COMMANDS["uname"] = uname
    COMMANDS["env"] = env_cmd
    COMMANDS["whoami"] = whoami
    COMMANDS["link"] = link
    COMMANDS["unlink"] = unlink
    COMMANDS["tee"] = tee
    COMMANDS["mkdir"] = mkdir
    COMMANDS["rmdir"] = rmdir
    COMMANDS["hostname"] = hostname
    COMMANDS["logname"] = logname
    COMMANDS["readlink"] = readlink
    COMMANDS["realpath"] = realpath
    COMMANDS["uniq"] = uniq
    COMMANDS["uptime"] = uptime
    COMMANDS["id"] = id
    COMMANDS["who"] = who
    COMMANDS["kill"] = kill
    COMMANDS["cut"] = cut
    COMMANDS["tr"] = tr
    COMMANDS["mv"] = mv
    COMMANDS["rm"] = rm
    COMMANDS["ln"] = ln
    COMMANDS["chmod"] = chmod
    COMMANDS["chown"] = chown
    COMMANDS["sort"] = sort
    COMMANDS["grep"] = grep
    COMMANDS["ls"] = ls
    COMMANDS["cp"] = cp
    COMMANDS["printf"] = printf
    COMMANDS["date"] = date
    COMMANDS["expr"] = expr
    COMMANDS["split"] = split
    COMMANDS["tail"] = tail
    COMMANDS["tac"] = tac
    COMMANDS["fold"] = fold
    COMMANDS["expand"] = expand
    COMMANDS["rev"] = rev
    COMMANDS["comm"] = comm
    COMMANDS["join"] = join
    COMMANDS["nl"] = nl
    COMMANDS["shuf"] = shuf
    COMMANDS["sum"] = sum
    COMMANDS["du"] = du
    COMMANDS["df"] = df
    COMMANDS["test"] = test_cmd
    COMMANDS["["] = test_cmd

    cmd_name = get_command_name()
    argv0 = os.path.basename(sys.argv[0])

    if cmd_name in COMMANDS:
        if argv0 == cmd_name:
            args = [cmd_name, *sys.argv[1:]] if cmd_name == "[" else sys.argv[1:]
        elif len(sys.argv) > 1 and sys.argv[1] == cmd_name:
            args = [cmd_name, *sys.argv[2:]] if cmd_name == "[" else sys.argv[2:]
        else:
            args = sys.argv[1:]

        try:
            result = COMMANDS[cmd_name].run(args)
            sys.exit(result)
        except Exception as e:
            print(f"{cmd_name}: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        print(f"gvibu-ref: {cmd_name}: command not found", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    dispatch()
