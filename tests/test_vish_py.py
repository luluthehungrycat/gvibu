import os
import subprocess
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), os.pardir))
VISH_PY = os.path.join(REPO_ROOT, "gvibu-python", "vish.py")


def run(args, stdin=None):
    cmd = [sys.executable, VISH_PY] + list(args)
    result = subprocess.run(
        cmd,
        stdin=subprocess.PIPE if stdin is not None else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        input=stdin,
    )
    return result.stdout, result.stderr, result.returncode


def test_help():
    out, err, code = run(["help"])
    assert code == 0
    assert "gvibu" in out
    assert "Built-in commands" in out


def test_version():
    out, err, code = run(["version"])
    assert code == 0
    assert out.startswith("gvibu v")


def test_echo():
    out, err, code = run(["echo", "hello"])
    assert code == 0
    assert out == "hello"


def test_true():
    out, err, code = run(["true"])
    assert code == 0


def test_false():
    out, err, code = run(["false"])
    assert code == 1


def test_unknown_command():
    out, err, code = run(["nonexistent-command-xyz"])
    assert code == 1
    assert "command not found" in err
