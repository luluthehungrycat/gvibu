import subprocess
import sys


def run(args):
    cmd = [sys.executable, "gvibu-python/vish.py"] + args
    result = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
    )
    return result.stdout.strip(), result.returncode


def test_integration_help():
    out, code = run(["help"])
    assert code == 0
    assert "GVIBU Python shell" in out


def test_integration_gvibu_init():
    out, code = run(["gvibu", "init"])
    assert code == 0
    assert "GVIBU Python init" in out or "GVIBU init" in out
