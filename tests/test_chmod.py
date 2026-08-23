"""Tests for chmod command."""
import pytest
from gvibu_ref.commands.chmod import (
    run, _parse_octal, _parse_symbolic, _apply_mode_change
)


def test_parse_octal():
    assert _parse_octal("755") == 0o755
    assert _parse_octal("644") == 0o644
    assert _parse_octal("0644") == 0o644
    assert _parse_octal("777") == 0o777
    assert _parse_octal("") is None
    assert _parse_octal("8") is None
    assert _parse_octal("abc") is None


def test_parse_symbolic():
    result = _parse_symbolic("u+x")
    assert result is not None
    who, op, bits = result
    assert 'u' in who
    assert op == '+'
    assert bits == 0o100

    result = _parse_symbolic("go-w")
    assert result is not None
    who, op, bits = result
    assert 'g' in who and 'o' in who
    assert op == '-'
    assert bits == 0o022


def test_apply_mode_change():
    assert _apply_mode_change(0o644, "u", '+', 0o100) == 0o744
    assert _apply_mode_change(0o755, "g", '-', 0o010) == 0o745
    assert _apply_mode_change(0o644, "a", '=', 0o444) == 0o444


def test_no_args():
    assert run([]) == 1


def test_invalid_option():
    assert run(["-x", "755", "/dev/null"]) == 1


def test_no_files():
    assert run(["755"]) == 1


def test_dev_null_octal():
    assert run(["644", "/dev/null"]) == 1


def test_dev_null_symbolic():
    assert run(["u+x", "/dev/null"]) == 1


def test_dev_null_recursive():
    assert run(["-R", "755", "/dev/null"]) == 1


def test_dev_null_verbose():
    assert run(["-v", "644", "/dev/null"]) == 1
