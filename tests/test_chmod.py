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


def test_regular_file_octal(tmp_path):
    path = tmp_path / "file"
    path.write_text("content")
    assert run(["644", str(path)]) == 0
    assert path.stat().st_mode & 0o777 == 0o644


def test_regular_file_symbolic(tmp_path):
    path = tmp_path / "file"
    path.write_text("content")
    assert run(["u+x", str(path)]) == 0
    assert path.stat().st_mode & 0o700 == 0o700


def test_regular_directory_recursive(tmp_path):
    nested = tmp_path / "nested"
    nested.mkdir()
    child = nested / "file"
    child.write_text("content")
    assert run(["-R", "755", str(nested)]) == 0
    assert nested.stat().st_mode & 0o777 == 0o755
    assert child.stat().st_mode & 0o777 == 0o755


def test_regular_file_verbose(tmp_path, capsys):
    path = tmp_path / "file"
    path.write_text("content")
    assert run(["-v", "644", str(path)]) == 0
    assert str(path) in capsys.readouterr().out
