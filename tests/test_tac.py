"""Tests for tac command."""

from gvibu_ref.commands import tac


def test_stdin_basic(capsys):
    ret = tac.run([])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_dev_null(capsys):
    ret = tac.run(["/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out == ""
    assert err == ""


def test_invalid_option(capsys):
    ret = tac.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_file_reversed(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("a\nb\nc\n")
    ret = tac.run([str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.strip().split("\n") == ["c", "b", "a"]


def test_nonexistent_file(capsys):
    ret = tac.run(["/nonexistent_tac_test_xyz"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""
