"""Tests for sum."""
import tempfile
from gvibu_ref.commands.sum import run, bsd_checksum


def test_bsd_checksum_empty():
    assert bsd_checksum(b"") == 0


def test_bsd_checksum_nonempty():
    assert bsd_checksum(b"abc") != 0


def test_no_args():
    assert run([]) == 1


def test_nonexistent_file():
    assert run(["/nonexistent_sum_test_xyz"]) == 1


def test_dev_null():
    assert run(["/dev/null"]) == 0


def test_tmpfile():
    with tempfile.NamedTemporaryFile(mode="w", delete=False) as f:
        f.write("hello\n")
        fname = f.name
    try:
        assert run([fname]) == 0
    finally:
        import os
        os.unlink(fname)
