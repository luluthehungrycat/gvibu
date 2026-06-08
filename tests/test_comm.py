"""Tests for comm command."""
import subprocess
import sys


def test_comm_no_args():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "comm"],
        capture_output=True, text=True,
    )
    assert result.returncode == 1
    assert "missing operand" in result.stderr


def test_comm_invalid_option():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "comm", "-x", "/dev/null", "/dev/null"],
        capture_output=True, text=True,
    )
    assert result.returncode == 1
    assert "invalid option" in result.stderr
