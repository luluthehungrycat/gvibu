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
    assert "GVIBU minimal shell" in out


def test_gvibu_help():
    out, code = run(["./gvibu-linux/vish", "gvibu", "help"])
    assert code == 0
    assert "GVIBU command surface" in out


def test_init():
    out, code = run(["./gvibu-linux/vish", "init"])
    assert code == 0
    assert "GVIBU init" in out


def test_status():
    out, code = run(["./gvibu-linux/vish", "status"])
    assert code == 0
    assert "GVIBU status" in out


def test_unknown():
    out, code = run(["./gvibu-linux/vish", "not-a-command"])
    assert code != 0
    assert "Unknown command" in out
