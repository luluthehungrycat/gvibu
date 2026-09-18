"""Tests for grep command."""

import io
import sys

from gvibu_ref.commands import grep


def test_missing_pattern(capsys):
    ret = grep.run([])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_invalid_option(capsys):
    ret = grep.run(["-x", "pattern"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_invalid_pattern(capsys):
    ret = grep.run(["[invalid"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_no_match_stdin(capsys, monkeypatch):
    monkeypatch.setattr(sys, "stdin", io.StringIO(""))
    ret = grep.run(["xyz"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err == ""


def test_file_match(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("hello world\nfoo bar\nhello again\n")
    ret = grep.run(["hello", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    lines = out.strip().split("\n")
    assert len(lines) == 2
    assert lines[0] == "hello world"
    assert lines[1] == "hello again"


def test_file_no_match(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("hello world\n")
    ret = grep.run(["xyz", str(f)])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err == ""


def test_ignore_case(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("Hello World\n")
    ret = grep.run(["-i", "hello", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.strip() == "Hello World"


def test_invert(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("keep\nskip\nkeep2\n")
    ret = grep.run(["-v", "skip", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.strip().split("\n") == ["keep", "keep2"]


def test_count(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("hello\nworld\nhello\n")
    ret = grep.run(["-c", "hello", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.strip() == "2"


def test_line_number(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("first\nsecond\nthird\n")
    ret = grep.run(["-n", "second", str(f)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "2:" in out
    assert "second" in out


def test_files_with_matches(tmp_path, capsys):
    f1 = tmp_path / "match.txt"
    f2 = tmp_path / "nomatch.txt"
    f1.write_text("hello\n")
    f2.write_text("world\n")
    ret = grep.run(["-l", "hello", str(f1), str(f2)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "match.txt" in out
    assert "nomatch.txt" not in out


def test_bundled_flags(tmp_path, capsys):
    f = tmp_path / "test.txt"
    f.write_text("Hello World\n")
    ret = grep.run(["-iv", "hello", str(f)])
    out, err = capsys.readouterr()
    assert ret == 1  # -iv = invert + ignore: "hello" matches "Hello" so inverted = no output
    assert out == ""
    assert err == ""


def test_recursive(tmp_path, capsys):
    sub = tmp_path / "subdir"
    sub.mkdir()
    (sub / "file.txt").write_text("match this\n")
    ret = grep.run(["-r", "match", str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "match" in out


def test_nonexistent_file(capsys):
    ret = grep.run(["pattern", "/nonexistent_grep_test_xyz"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_multiple_files(tmp_path, capsys):
    f1 = tmp_path / "a.txt"
    f2 = tmp_path / "b.txt"
    f1.write_text("common\n")
    f2.write_text("common\n")
    ret = grep.run(["common", str(f1), str(f2)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    lines = out.strip().split("\n")
    assert len(lines) == 2
    assert all("common" in l for l in lines)


def test_multiple_files_line_number(tmp_path, capsys):
    f1 = tmp_path / "a.txt"
    f2 = tmp_path / "b.txt"
    f1.write_text("line1\nmatch\n")
    f2.write_text("match\n")
    ret = grep.run(["-n", "match", str(f1), str(f2)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    lines = out.strip().split("\n")
    assert any(line.endswith("a.txt:2:match") for line in lines)
    assert any(line.endswith("b.txt:1:match") for line in lines)
