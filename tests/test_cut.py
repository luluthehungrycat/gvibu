from gvibu_ref.commands.cut import run as cut_run


def test_cut_invalid_option(capfd):
    code = cut_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "cut:" in err


def test_cut_no_f_flag(capfd):
    code = cut_run(["-d", ","])
    assert code == 1
    out, err = capfd.readouterr()
    assert "cut:" in err


def test_cut_missing_d_arg(capfd):
    code = cut_run(["-d"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "cut:" in err


def test_cut_missing_f_arg(capfd):
    code = cut_run(["-f"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "cut:" in err
