import subprocess
import sys


def run(args):
    cmd = [sys.executable, "gvibu-python/vish.py"] + args
    result = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
    )
    return result.stdout.strip(), result.returncode


def test_help_py():
    out, code = run(["help"])
    assert code == 0
    assert "GVIBU Python shell" in out


def test_init_py():
    out, code = run(["init"])
    assert code == 0
    assert "GVIBU init" in out


def test_status_py():
    out, code = run(["status"])
    assert code == 0
    assert "GVIBU status" in out
