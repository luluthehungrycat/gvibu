"""Tests for printf command."""

from gvibu_ref.commands import printf


def test_no_args(capsys):
    ret = printf.run([])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_string_no_format(capsys):
    ret = printf.run(["hello\n"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "hello\n"


def test_s_specifier(capsys):
    ret = printf.run(["%s\n", "hello"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "hello\n"


def test_d_specifier(capsys):
    ret = printf.run(["%d\n", "42"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "42\n"


def test_x_specifier(capsys):
    ret = printf.run(["%x\n", "255"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "ff\n"


def test_x_upper_specifier(capsys):
    ret = printf.run(["%X\n", "255"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "FF\n"


def test_o_specifier(capsys):
    ret = printf.run(["%o\n", "255"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    # 255 decimal = 377 octal
    assert out == "377\n"


def test_u_specifier(capsys):
    ret = printf.run(["%u\n", "42"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "42\n"


def test_percent(capsys):
    ret = printf.run(["%%\n"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "%\n"


def test_c_specifier(capsys):
    ret = printf.run(["%c\n", "A"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "A\n"


def test_tab_escape(capsys):
    ret = printf.run(["a\\tb"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "a\tb"


def test_newline_escape(capsys):
    ret = printf.run(["a\\nb"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "a\nb"


def test_octal_escape(capsys):
    ret = printf.run(["\\0101\\n"])  # 101 octal = 65 = 'A'
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "A\n"


def test_width_specifier(capsys):
    ret = printf.run(["%10s\n", "hi"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "        hi\n"


def test_multiple_args(capsys):
    ret = printf.run(["%s %s\n", "hello", "world"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "hello world\n"


def test_arg_recycling(capsys):
    ret = printf.run(["%d %d %d\n", "1"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out == "1 1 1\n"


def test_interpret_escapes():
    assert printf.interpret_escapes("a\\tb") == "a\tb"
    assert printf.interpret_escapes("a\\nb") == "a\nb"
    assert printf.interpret_escapes("a\\\\b") == "a\\b"
    assert printf.interpret_escapes("\\0101") == "A"
    assert printf.interpret_escapes("hello") == "hello"
