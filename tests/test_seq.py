"""Tests for seq command."""

from gvibu_ref.commands.seq import run as seq_run


def test_seq_basic(capsys):
    result = seq_run(["5"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "1\n2\n3\n4\n5\n"


def test_seq_first_last(capsys):
    result = seq_run(["3", "7"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "3\n4\n5\n6\n7\n"


def test_seq_first_step_last(capsys):
    result = seq_run(["2", "3", "14"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "2\n5\n8\n11\n14\n"


def test_seq_first_greater_than_last():
    result = seq_run(["10", "5"])
    assert result == 0


def test_seq_negative_step(capsys):
    result = seq_run(["10", "-2", "4"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "10\n8\n6\n4\n"


def test_seq_step_zero():
    result = seq_run(["1", "0", "5"])
    assert result == 1


def test_seq_no_args():
    result = seq_run([])
    assert result == 1


def test_seq_invalid_arg():
    result = seq_run(["abc"])
    assert result == 1


def test_seq_with_separator(capsys):
    result = seq_run(["-s", ",", "3"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "1,2,3\n"


def test_seq_equal_width(capsys):
    result = seq_run(["-w", "5", "10"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "05\n06\n07\n08\n09\n10\n"


def test_seq_w_and_s(capsys):
    result = seq_run(["-w", "-s", " ", "3"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "1 2 3\n"
