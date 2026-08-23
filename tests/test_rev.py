"""Tests for rev command."""
import subprocess
import sys


def run_cli(*args: str, stdin: str = ""):
    return subprocess.run(
        [sys.executable, "python-ref/gvibu_ref/main.py", "rev", *args],
        input=stdin,
        capture_output=True,
        text=True,
    )


def test_rev_stdin_lines():
    result = run_cli(stdin="abc\nRust 2026\n\n")
    assert result.returncode == 0
    assert result.stdout == "cba\n6202 tsuR\n\n"
    assert result.stderr == ""


def test_rev_rejects_unknown_option():
    result = run_cli("-x")
    assert result.returncode == 1
    assert result.stdout == ""
    assert "invalid option" in result.stderr
