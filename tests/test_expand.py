"""Tests for expand command."""
import subprocess
import sys


def run_cli(*args: str, stdin: str = ""):
    return subprocess.run(
        [sys.executable, "python-ref/gvibu_ref/main.py", "expand", *args],
        input=stdin,
        capture_output=True,
        text=True,
    )


def test_expand_default_tabs():
    result = run_cli(stdin="\tfoo\nab\tcd\n")
    assert result.returncode == 0
    assert result.stdout == "        foo\nab      cd\n"
    assert result.stderr == ""


def test_expand_custom_tab_stops():
    result = run_cli("-t", "4", stdin="ab\tcd\n")
    assert result.returncode == 0
    assert result.stdout == "ab  cd\n"
    assert result.stderr == ""
