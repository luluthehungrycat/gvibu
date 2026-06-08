from gvibu_ref.commands.tr import run as tr_run


def test_tr_no_args(capfd):
    code = tr_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "tr:" in err


def test_tr_invalid_option(capfd):
    code = tr_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "tr:" in err
