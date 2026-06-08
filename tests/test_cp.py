"""Tests for cp command."""

import os
import tempfile

from gvibu_ref.commands import cp


def test_no_args(capsys):
    ret = cp.run([])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_one_arg(capsys):
    ret = cp.run(["file.txt"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_invalid_option(capsys):
    ret = cp.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_nonexistent_source(capsys):
    ret = cp.run(["/nonexistent_cp_test_xyz", "/tmp/cp_dest"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_copy_file(tmp_path, capsys):
    src = tmp_path / "src.txt"
    src.write_text("hello")
    dst = tmp_path / "dst.txt"
    ret = cp.run([str(src), str(dst)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert dst.read_text() == "hello"


def test_verbose_flag(tmp_path, capsys):
    src = tmp_path / "src.txt"
    src.write_text("data")
    dst = tmp_path / "dst.txt"
    ret = cp.run(["-v", str(src), str(dst)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "'src.txt' -> 'dst.txt'" in out or f"'{src}' -> '{dst}'" in out


def test_no_clobber(tmp_path, capsys):
    src = tmp_path / "src.txt"
    src.write_text("new data")
    dst = tmp_path / "dst.txt"
    dst.write_text("old data")
    ret = cp.run(["-n", str(src), str(dst)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert dst.read_text() == "old data"


def test_copy_to_dir(tmp_path, capsys):
    src = tmp_path / "file.txt"
    src.write_text("content")
    subdir = tmp_path / "subdir"
    subdir.mkdir()
    ret = cp.run([str(src), str(subdir)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert (subdir / "file.txt").read_text() == "content"


def test_recursive_copy(tmp_path, capsys):
    srcdir = tmp_path / "srcdir"
    srcdir.mkdir()
    (srcdir / "a.txt").write_text("a")
    (srcdir / "b.txt").write_text("b")
    sub = srcdir / "sub"
    sub.mkdir()
    (sub / "c.txt").write_text("c")
    dstdir = tmp_path / "dstdir"
    ret = cp.run(["-r", str(srcdir), str(dstdir)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert (dstdir / "a.txt").read_text() == "a"
    assert (dstdir / "b.txt").read_text() == "b"
    assert (dstdir / "sub" / "c.txt").read_text() == "c"


def test_omit_directory_no_r(tmp_path, capsys):
    ret = cp.run([str(tmp_path), "/tmp/nonexistent_cp_test"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert "omitting directory" in err
