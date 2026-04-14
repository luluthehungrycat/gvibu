import os
import sys


def test_basename_no_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.basename import run as basename_run

    code = basename_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "usage" in err.lower()


def test_basename_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.basename import run as basename_run

    code = basename_run(["/usr/local/bin/testfile.txt", ".txt"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out.strip() == "testfile"
    assert err == ""
