"""Tests for seq command."""

from gvibu_ref.commands import seq


def test_seq_basic(capsys):
    result = seq.run(["5"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "1\n2\n3\n4\n5\n"


def test_seq_first_last(capsys):
    result = seq.run(["3", "7"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "3\n4\n5\n6\n7\n"


def test_seq_first_step_last(capsys):
    result = seq.run(["2", "3", "14"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "2\n5\n8\n11\n14\n"


def test_seq_first_greater_than_last():
    result = seq.run(["10", "5"])
    assert result == 0


def test_seq_negative_step(capsys):
    result = seq.run(["10", "-2", "4"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == "10\n8\n6\n4\n"


def test_seq_step_zero():
    result = seq.run(["1", "0", "5"])
    assert result == 1


def test_seq_no_args():
    result = seq.run([])
    assert result == 1


def test_seq_invalid_arg():
    result = seq.run(["abc"])
    assert result == 1
