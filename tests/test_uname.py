"""Tests for uname command."""

from gvibu_ref.commands import uname


def test_uname_default(capsys):
    result = uname.run([])
    out, _ = capsys.readouterr()
    assert result == 0
    assert len(out.strip()) > 0


def test_uname_s(capsys):
    result = uname.run(["-s"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out.strip() == "Linux"


def test_uname_n(capsys):
    result = uname.run(["-n"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert len(out.strip()) > 0


def test_uname_r(capsys):
    result = uname.run(["-r"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert len(out.strip()) > 0


def test_uname_m(capsys):
    result = uname.run(["-m"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert len(out.strip()) > 0


def test_uname_all(capsys):
    result = uname.run(["-a"])
    out, _ = capsys.readouterr()
    assert result == 0
    parts = out.strip().split()
    assert len(parts) >= 4


def test_uname_invalid_option():
    result = uname.run(["-x"])
    assert result == 1
