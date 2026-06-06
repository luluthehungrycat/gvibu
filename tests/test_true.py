from gvibu_ref.commands.true import run as true_run


def test_true_no_args(capfd):
    code = true_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_true_with_args(capfd):
    code = true_run(["a", "b"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""
