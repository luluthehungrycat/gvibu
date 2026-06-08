"""Integration tests for join command."""

import subprocess
import sys


def test_join_no_args():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "join"],
        capture_output=True, text=True
    )
    assert result.returncode == 1
    assert "missing operand" in result.stderr


def test_join_one_arg():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "join", "file1"],
        capture_output=True, text=True
    )
    assert result.returncode == 1
    assert "missing operand" in result.stderr


def test_join_invalid_option():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "join", "-x"],
        capture_output=True, text=True
    )
    assert result.returncode == 1
    assert "invalid option" in result.stderr
