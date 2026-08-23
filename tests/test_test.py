"""Tests for test/[ command."""
import subprocess
import sys

import pytest
from gvibu_ref.commands.test_cmd import run


def test_cli_bracket_missing_close():
    result = subprocess.run(
        [sys.executable, "python-ref/gvibu_ref/main.py", "[", "x", "=", "x"],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 1
    assert result.stdout == ""
    assert "missing ']'" in result.stderr


def test_no_args():
    assert run([]) == 1


def test_string_nonempty():
    assert run(["hello"]) == 0


def test_n_flag_nonempty():
    assert run(["-n", "hello"]) == 0


def test_z_flag_empty():
    assert run(["-z", ""]) == 0


def test_eq_true():
    assert run(["a", "=", "a"]) == 0


def test_eq_false():
    assert run(["a", "=", "b"]) == 1


def test_neq_true():
    assert run(["a", "!=", "b"]) == 0


def test_num_eq_true():
    assert run(["5", "-eq", "5"]) == 0


def test_num_eq_false():
    assert run(["5", "-eq", "6"]) == 1


def test_num_lt_true():
    assert run(["3", "-lt", "5"]) == 0


def test_num_lt_false():
    assert run(["5", "-lt", "3"]) == 1


def test_num_gt_true():
    assert run(["7", "-gt", "2"]) == 0


def test_num_le_true():
    assert run(["3", "-le", "3"]) == 0


def test_num_ge_true():
    assert run(["5", "-ge", "5"]) == 0


def test_not_empty():
    assert run(["!", ""]) == 0


def test_bracket_form_true():
    assert run(["[", "hello", "]"]) == 0


def test_bracket_form_false():
    assert run(["[", "", "]"]) == 1


def test_bracket_missing_close():
    assert run(["[", "hello"]) == 1


def test_dev_null_exists():
    assert run(["-e", "/dev/null"]) == 0


def test_regular_file_is_file(tmp_path):
    path = tmp_path / "regular-file"
    path.write_text("content")
    assert run(["-f", str(path)]) == 0


def test_root_is_dir():
    assert run(["-d", "/"]) == 0
