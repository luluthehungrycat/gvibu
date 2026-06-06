import os

from gvibu_ref.commands.pwd import run as pwd_run


def test_pwd_no_args(capfd):
    code = pwd_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == os.getcwd()
    assert err == ""


def test_pwd_with_args(capfd):
    code = pwd_run(["extra"])
    assert code == 2
    out, err = capfd.readouterr()
    assert "usage" in err.lower()
