from gvibu_ref.commands.hostname import run as hostname_run


def test_hostname_no_args(capfd):
    code = hostname_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert len(out) > 0
    assert out.endswith("\n")
    assert err == ""


def test_hostname_with_args(capfd):
    code = hostname_run(["extra"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "hostname:" in err
