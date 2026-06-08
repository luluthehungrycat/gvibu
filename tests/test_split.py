"""Tests for split command."""

import os

from gvibu_ref.commands import split


def test_invalid_option(capsys):
    ret = split.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_missing_a_arg(capsys):
    ret = split.run(["-a"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_invalid_a_value(capsys):
    ret = split.run(["-a", "0"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_missing_l_arg(capsys):
    ret = split.run(["-l"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_invalid_lines(capsys):
    ret = split.run(["-l", "0"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_make_suffix_alpha():
    assert split.make_suffix("x", 0, False, 2) == "xaa"
    assert split.make_suffix("x", 1, False, 2) == "xab"
    assert split.make_suffix("x", 25, False, 2) == "xaz"
    assert split.make_suffix("x", 26, False, 2) == "xba"


def test_make_suffix_numeric():
    assert split.make_suffix("x", 0, True, 2) == "x00"
    assert split.make_suffix("x", 1, True, 2) == "x01"
    assert split.make_suffix("x", 99, True, 2) == "x99"


def test_split_stdin(tmp_path, capsys):
    """Test split creates xaa from stdin."""
    orig_cwd = os.getcwd()
    os.chdir(tmp_path)
    try:
        import sys
        from io import StringIO
        old_stdin = sys.stdin
        sys.stdin = StringIO("line1\nline2\nline3\n")
        ret = split.run([])
        sys.stdin = old_stdin
        out, err = capsys.readouterr()
        assert ret == 0
        assert err == ""
        assert os.path.exists("xaa")
        with open("xaa") as f:
            content = f.read()
        assert "line1" in content
    finally:
        os.chdir(orig_cwd)


def test_split_lines_flag(tmp_path, capsys):
    """Test -l flag splits correctly."""
    orig_cwd = os.getcwd()
    os.chdir(tmp_path)
    try:
        with open("test.txt", "w") as f:
            for i in range(10):
                f.write(f"line{i}\n")
        import sys
        from io import StringIO
        old_stdin = sys.stdin
        sys.stdin = StringIO("")
        ret = split.run(["-l", "3", "test.txt"])
        sys.stdin = old_stdin
        out, err = capsys.readouterr()
        assert ret == 0
        assert err == ""
        # Should create xaa (3 lines), xab (3 lines), xac (3 lines), xad (1 line)
        assert os.path.exists("xaa")
        assert os.path.exists("xab")
        assert os.path.exists("xac")
        assert os.path.exists("xad")
        with open("xaa") as f:
            assert len(f.readlines()) == 3
    finally:
        os.chdir(orig_cwd)
