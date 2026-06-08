from gvibu_ref.commands.realpath import run as realpath_run


def test_realpath_no_args(capfd):
    code = realpath_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "realpath:" in err


def test_realpath_too_many(capfd):
    code = realpath_run(["a", "b"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "realpath:" in err
