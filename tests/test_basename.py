from gvibu_ref.commands.basename import run as basename_run


def test_basename_no_args(capfd):
    code = basename_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "usage" in err.lower()


def test_basename_args(capfd):
    code = basename_run(["/usr/local/bin/testfile.txt", ".txt"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "testfile"
    assert err == ""
