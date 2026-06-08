from gvibu_ref.commands.uptime import run as uptime_run


def test_uptime_no_args(capfd):
    code = uptime_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert len(out) > 0
    assert out.startswith("up ")
    assert err == ""


def test_uptime_with_args(capfd):
    code = uptime_run(["extra"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "uptime:" in err
