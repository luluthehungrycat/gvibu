from gvibu_ref.commands.who import run as who_run


def test_who_invalid_option(capfd):
    code = who_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "who:" in err


def test_who_no_args(capfd):
    code = who_run([])
    # Should succeed or say no users logged in
    assert code == 0
