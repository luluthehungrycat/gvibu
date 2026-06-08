from gvibu_ref.commands.readlink import run as readlink_run


def test_readlink_no_args(capfd):
    code = readlink_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "readlink:" in err


def test_readlink_too_many(capfd):
    code = readlink_run(["a", "b"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "readlink:" in err
