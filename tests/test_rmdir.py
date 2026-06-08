import os
import tempfile

from gvibu_ref.commands.rmdir import run as rmdir_run


def test_rmdir_no_args(capfd):
    code = rmdir_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "rmdir:" in err


def test_rmdir_nonexistent(capfd):
    code = rmdir_run(["/tmp/nonexistent_rmdir_test_xyz"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "rmdir:" in err


def test_rmdir_success(capfd):
    with tempfile.TemporaryDirectory() as tmpdir:
        d = os.path.join(tmpdir, "subdir")
        os.mkdir(d)
        assert os.path.isdir(d)
        code = rmdir_run([d])
        assert code == 0
        out, err = capfd.readouterr()
        assert err == ""
        assert not os.path.exists(d)
