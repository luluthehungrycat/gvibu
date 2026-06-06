"""Tests for yes command."""
import subprocess
import sys
import os

MAIN_SCRIPT = os.path.join(os.path.dirname(__file__), "..", "python-ref", "gvibu_ref", "main.py")


def test_yes_default():
    """yes without args should output 'y' repeatedly."""
    proc = subprocess.Popen(
        [sys.executable, MAIN_SCRIPT, "yes"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
    )
    # Read first 3 lines, then kill
    lines = []
    for _ in range(3):
        line = proc.stdout.readline()
        lines.append(line.strip())
    proc.kill()
    proc.wait()

    assert lines == ["y", "y", "y"]


def test_yes_with_strings():
    """yes with args should repeat those args."""
    proc = subprocess.Popen(
        [sys.executable, MAIN_SCRIPT, "yes", "hello", "world"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
    )
    lines = []
    for _ in range(2):
        line = proc.stdout.readline()
        lines.append(line.strip())
    proc.kill()
    proc.wait()

    assert lines == ["hello world", "hello world"]
