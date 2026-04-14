import os
import sys


def test_pwd_no_args(capfd):
    # Ensure Python can import the Python reference implementation
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.pwd import run as pwd_run

    code = pwd_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == os.getcwd()
    assert err == ""


def test_pwd_with_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.pwd import run as pwd_run

    code = pwd_run(["extra"])
    assert code == 2
    out, err = capfd.readouterr()
    assert "usage" in err.lower()
