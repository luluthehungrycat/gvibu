"""Tests for tail command."""

import io
import sys

from gvibu_ref.commands import tail

def test_no_args_stdin(capsys, monkeypatch):
    monkeypatch.setattr(sys, "stdin", io.StringIO("a\nb\nc\n"))
    ret = tail.run([])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_n_flag_stdin(capsys, monkeypatch):
    monkeypatch.setattr(sys, "stdin", io.StringIO("a\nb\nc\nd\n"))
    ret = tail.run(["-n", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_c_flag_stdin(capsys, monkeypatch):
    monkeypatch.setattr(sys, "stdin", io.TextIOWrapper(io.BytesIO(b"hello world")))
    ret = tail.run(["-c", "5"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_dev_null(capsys):
    ret = tail.run(["/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out == ""
    assert err == ""


def test_invalid_option(capsys):
    ret = tail.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_c_missing_arg(capsys):
    ret = tail.run(["-c"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_invalid_c_value(capsys):
    ret = tail.run(["-c", "abc"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_nonexistent_file(capsys):
    ret = tail.run(["/nonexistent_tail_test_xyz"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_file_last_10(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("\n".join(str(i) for i in range(20)) + "\n")
    ret = tail.run([str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    lines = out.strip().split("\n")
    assert len(lines) == 10
    assert lines[0] == "10"


def test_file_last_3(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("a\nb\nc\nd\ne\n")
    ret = tail.run(["-n", "3", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.strip().split("\n") == ["c", "d", "e"]


def test_file_last_bytes(tmp_path, capsys):
    f = tmp_path / "test.bin"
    f.write_bytes(b"hello world")
    ret = tail.run(["-c", "5", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "world"
