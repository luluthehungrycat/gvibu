from gvibu_ref.commands.mv import run as mv_run


def test_mv_no_args(capfd):
    code = mv_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "mv:" in err


def test_mv_one_arg(capfd):
    code = mv_run(["a"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "mv:" in err


def test_mv_invalid_option(capfd):
    code = mv_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "mv:" in err
