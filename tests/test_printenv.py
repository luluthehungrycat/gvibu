"""Tests for printenv command."""
from gvibu_ref.commands.printenv import run as printenv_run
import os


def test_unknown_var(capfd):
    """printenv for a nonexistent var should print empty line."""
    code = printenv_run(["__GVIBU_NONEXISTENT_VAR_XYZ__"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "\n"
    assert err == ""


def test_known_var(capfd):
    """printenv for PATH should print the PATH value."""
    code = printenv_run(["PATH"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == os.environ.get("PATH", "")
    assert err == ""


def test_multiple_vars(capfd):
    """printenv with multiple names should print each on its own line."""
    code = printenv_run(["PATH", "HOME"])
    assert code == 0
    out, err = capfd.readouterr()
    lines = out.strip().split("\n")
    assert len(lines) == 2
    assert lines[0] == os.environ.get("PATH", "")
    assert lines[1] == os.environ.get("HOME", "")


def test_all_vars(capfd):
    """printenv without args should print all env vars."""
    code = printenv_run([])
    assert code == 0
    out, err = capfd.readouterr()
    # Should be at least as many lines as there are env vars
    lines = out.strip().split("\n")
    assert len(lines) >= len(os.environ)
    assert err == ""
