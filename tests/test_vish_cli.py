import subprocess
import os


def run(args):
    # Ensure the script path is correct relative to repo root
    if isinstance(args, list):
        cmd = args
    else:
        cmd = [args]
    result = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
    )
    return result.stdout.strip(), result.returncode


def test_help():
    out, code = run(["./gvibu-linux/vish", "help"])
    assert code == 0
    assert "Built-in commands:" in out


def test_command_dispatch():
    out, code = run(["./gvibu-linux/vish", "echo", "hello"])
    assert code == 0
    assert out == "hello"


def test_unknown():
    out, code = run(["./gvibu-linux/vish", "not-a-command"])
    assert code != 0
    assert "command not found" in out
