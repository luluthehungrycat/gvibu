"""Tests for expr command."""

from gvibu_ref.commands import expr


def test_no_args(capsys):
    ret = expr.run([])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_number(capsys):
    ret = expr.run(["42"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "42"
    assert err == ""


def test_addition(capsys):
    ret = expr.run(["1", "+", "2"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "3"


def test_subtraction(capsys):
    ret = expr.run(["10", "-", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "7"


def test_multiplication(capsys):
    ret = expr.run(["3", "*", "4"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "12"


def test_division(capsys):
    ret = expr.run(["10", "/", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "3"


def test_modulo(capsys):
    ret = expr.run(["10", "%", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "1"


def test_equality(capsys):
    ret = expr.run(["5", "=", "5"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "1"


def test_inequality(capsys):
    ret = expr.run(["5", "=", "6"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out.strip() == "0"


def test_not_equal(capsys):
    ret = expr.run(["5", "!=", "6"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "1"


def test_less_than(capsys):
    ret = expr.run(["3", "<", "5"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "1"


def test_greater_than(capsys):
    ret = expr.run(["5", ">", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "1"


def test_logical_or(capsys):
    ret = expr.run(["0", "|", "5"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "5"


def test_logical_and(capsys):
    ret = expr.run(["5", "&", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "5"


def test_length(capsys):
    ret = expr.run(["length", "hello"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "5"


def test_substr(capsys):
    ret = expr.run(["substr", "hello", "2", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "ell"


def test_index_found(capsys):
    ret = expr.run(["index", "hello", "l"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "3"


def test_index_not_found(capsys):
    ret = expr.run(["index", "hello", "x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out.strip() == "0"


def test_parentheses(capsys):
    ret = expr.run(["(", "1", "+", "2", ")", "*", "3"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "9"


def test_precedence(capsys):
    ret = expr.run(["2", "+", "3", "*", "4"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "14"


def test_string_compare(capsys):
    ret = expr.run(["abc", "=", "abc"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "1"


def test_division_by_zero(capsys):
    ret = expr.run(["1", "/", "0"])
    out, err = capsys.readouterr()
    assert ret == 2
    assert err != ""
