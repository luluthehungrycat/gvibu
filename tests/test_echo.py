from gvibu_ref.commands.echo import run as echo_run


def test_echo_no_args(capfd):
    code = echo_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "\n"
    assert err == ""


def test_echo_args(capfd):
    code = echo_run(["hello", "world"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "hello world\n"
    assert err == ""


def test_echo_no_newline(capfd):
    code = echo_run(["-n", "hello"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "hello"
    assert err == ""


def test_echo_no_newline_no_args(capfd):
    code = echo_run(["-n"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_echo_no_newline_multiple_args(capfd):
    code = echo_run(["-n", "hello", "world"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "hello world"
    assert err == ""


def test_echo_dash_n_not_first(capfd):
    code = echo_run(["hello", "-n", "world"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "hello -n world\n"
    assert err == ""


def test_echo_single_arg(capfd):
    code = echo_run(["hello"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "hello\n"
    assert err == ""
