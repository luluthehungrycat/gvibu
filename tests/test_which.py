"""Tests for which command."""

from gvibu_ref.commands import which_cmd


def test_which_found(capsys):
    # sh should always be in PATH
    result = which_cmd.run(["sh"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out.strip().endswith("/sh")


def test_which_not_found(capsys):
    result = which_cmd.run(["nonexistent_cmd_xyz"])
    out, _ = capsys.readouterr()
    assert result == 1
    assert out == ""


def test_which_multiple(capsys):
    result = which_cmd.run(["sh", "ls"])
    out, _ = capsys.readouterr()
    assert result == 0
    lines = out.strip().split("\n")
    assert len(lines) == 2
    assert lines[0].endswith("/sh")
    assert lines[1].endswith("/ls")


def test_which_no_args():
    result = which_cmd.run([])
    assert result == 1
