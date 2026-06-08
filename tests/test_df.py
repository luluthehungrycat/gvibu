"""Tests for df command."""

from gvibu_ref.commands import df


def test_invalid_option(capsys):
    ret = df.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_bundled_flags(capsys):
    ret = df.run(["-hT"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_no_args(capsys):
    ret = df.run([])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_human_readable(capsys):
    ret = df.run(["-h"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "Filesystem" in out
    # Check human-readable markers (K, M, G, T)
    assert any(c in out for c in ("K", "M", "G", "T"))


def test_with_type(capsys):
    ret = df.run(["-T"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "Type" in out


def test_specific_path(capsys):
    ret = df.run(["/"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "/" in out


def test_human_size_bytes():
    assert df._human_size(500) == "500"


def test_human_size_kb():
    assert df._human_size(2048) == "2.0K"


def test_human_size_mb():
    assert df._human_size(1048576 * 2) == "2.0M"
