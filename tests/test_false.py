from gvibu_ref.commands.false import run as false_run


def test_false_no_args(capfd):
    code = false_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_false_with_args(capfd):
    code = false_run(["foo", "bar"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""
