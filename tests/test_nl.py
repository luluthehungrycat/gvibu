"""Integration tests for nl command."""

import subprocess
import sys


def test_nl_dev_null():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "nl", "/dev/null"],
        capture_output=True, text=True
    )
    assert result.returncode == 0


def test_nl_no_args():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "nl"],
        capture_output=True, text=True
    )
    assert result.returncode == 0


def test_nl_invalid_option():
    result = subprocess.run(
        [sys.executable, "-m", "gvibu_ref", "nl", "-x"],
        capture_output=True, text=True
    )
    assert result.returncode == 1
    assert "invalid option" in result.stderr
