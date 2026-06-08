from gvibu_ref.commands.kill import run as kill_run


def test_kill_no_args(capfd):
    code = kill_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "kill:" in err


def test_kill_list_signals(capfd):
    code = kill_run(["-l"])
    assert code == 0
    out, err = capfd.readouterr()
    assert "TERM" in out


def test_kill_invalid_pid(capfd):
    code = kill_run(["abc"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "kill:" in err
