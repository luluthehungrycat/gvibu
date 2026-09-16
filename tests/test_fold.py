"""Tests for fold command."""
import subprocess
import sys


def test_fold_no_args_stdin_empty():
    """fold with no args and empty stdin should produce no output."""
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "fold"],
        input="",
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"stderr: {result.stderr}"
    assert result.stdout == ""


def test_fold_width_10():
    """fold -w 10 should wrap lines at column 10."""
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "fold", "-w", "10"],
        input="hello world this is a long line\n",
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"stderr: {result.stderr}"
    lines = result.stdout.splitlines()
    assert all(len(l) <= 10 for l in lines), f"lines too long: {lines}"


def test_fold_break_spaces():
    """fold -s -w 10 should break at spaces."""
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "fold", "-s", "-w", "10"],
        input="hello world this is a long line\n",
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"stderr: {result.stderr}"
    assert "hello" in result.stdout
    assert "world" in result.stdout


def test_fold_preserves_whitespace_without_s():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "fold", "-w", "5"],
        input="ab   cd",
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"stderr: {result.stderr}"
    assert result.stdout == "ab   \ncd\n"


def test_fold_invalid_width():
    """fold with invalid width should error."""
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "fold", "-w", "abc"],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 1
    assert "invalid width" in result.stderr


def test_fold_invalid_option():
    """fold with invalid option should error."""
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "fold", "-x"],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 1
    assert "invalid option" in result.stderr
