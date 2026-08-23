"""Tests for sort command."""

import io
import sys

from gvibu_ref.commands.sort import run


def test_no_args(monkeypatch):
    monkeypatch.setattr(sys, "stdin", io.StringIO(""))
    assert run([]) == 0


def test_dev_null():
    assert run(["/dev/null"]) == 0


def test_invalid_option():
    assert run(["-x"]) == 1


def test_combined_flags():
    assert run(["-rn", "/dev/null"]) == 0


def test_reverse_flag():
    assert run(["-r", "/dev/null"]) == 0


def test_numeric_flag():
    assert run(["-n", "/dev/null"]) == 0


def test_unique_flag():
    assert run(["-u", "/dev/null"]) == 0


def test_fold_case_flag():
    assert run(["-f", "/dev/null"]) == 0


def test_all_flags():
    assert run(["-rnuf", "/dev/null"]) == 0


def test_key_flag():
    assert run(["-k2,2n", "/dev/null"]) == 0


def test_key_flag_text():
    import tempfile, os
    content = "b 2\na 1\nc 3\n"
    with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
        f.write(content)
        f.flush()
        fname = f.name
    rc = run(["-k2,2n", fname])
    assert rc == 0
    os.unlink(fname)


def test_multi_key():
    import tempfile, os
    content = "a 3\na 1\na 2\n"
    with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
        f.write(content)
        f.flush()
        fname = f.name
    rc = run(["-k1,1", "-k2,2n", fname])
    assert rc == 0
    os.unlink(fname)
def test_version_sort_flag(tmp_path, capsys):
    path = tmp_path / "versions"
    path.write_text("pkg10\npkg2\npkg1\n")

    assert run(["-V", str(path)]) == 0

    assert capsys.readouterr().out == "pkg1\npkg2\npkg10\n"


def test_month_sort_long_flag(tmp_path, capsys):
    path = tmp_path / "months"
    path.write_text("Dec\nfoo\nJan\nFeb\n")

    assert run(["--month-sort", str(path)]) == 0

    assert capsys.readouterr().out == "foo\nJan\nFeb\nDec\n"


def test_check_flag_reports_disorder_without_output(tmp_path, capsys):
    ordered = tmp_path / "ordered"
    ordered.write_text("a\nb\n")
    unordered = tmp_path / "unordered"
    unordered.write_text("b\na\n")

    assert run(["-c", str(ordered)]) == 0
    assert capsys.readouterr().out == ""

    assert run(["--check", str(unordered)]) != 0
    assert capsys.readouterr().out == ""
