from gvibu_ref.commands.rm import run as rm_run


def test_rm_no_args(capfd):
    code = rm_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "rm:" in err


def test_rm_invalid_option(capfd):
    code = rm_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "rm:" in err
