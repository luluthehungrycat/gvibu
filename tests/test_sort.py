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
