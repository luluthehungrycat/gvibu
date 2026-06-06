from gvibu_ref.commands.whoami import run as whoami_run


def test_whoami_no_args(capfd):
    code = whoami_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert len(out) > 0
    assert out.endswith("\n")
    assert err == ""


def test_whoami_with_args(capfd):
    code = whoami_run(["extra"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "whoami:" in err
