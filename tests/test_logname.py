from gvibu_ref.commands.logname import run as logname_run


def test_logname_no_args(capfd):
    code = logname_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert len(out) > 0
    assert out.endswith("\n")
    assert err == ""


def test_logname_with_args(capfd):
    code = logname_run(["extra"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "logname:" in err
