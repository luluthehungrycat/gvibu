from gvibu_ref.commands.uniq import run as uniq_run


def test_uniq_invalid_option(capfd):
    code = uniq_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "uniq:" in err


def test_uniq_u_flag(capfd):
    code = uniq_run(["-u"])
    assert code == 0


def test_uniq_d_flag(capfd):
    code = uniq_run(["-d"])
    assert code == 0


def test_uniq_c_flag(capfd):
    code = uniq_run(["-c"])
    assert code == 0
