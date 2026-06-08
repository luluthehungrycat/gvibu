from gvibu_ref.commands.ln import run as ln_run


def test_ln_no_args(capfd):
    code = ln_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "ln:" in err


def test_ln_one_arg(capfd):
    code = ln_run(["a"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "ln:" in err


def test_ln_invalid_option(capfd):
    code = ln_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "ln:" in err
