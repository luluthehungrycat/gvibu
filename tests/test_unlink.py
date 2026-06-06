import os
import tempfile

from gvibu_ref.commands.unlink import run as unlink_run


def test_unlink_no_args(capfd):
    code = unlink_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "unlink:" in err


def test_unlink_too_many_args(capfd):
    code = unlink_run(["a", "b"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "unlink:" in err


def test_unlink_nonexistent(capfd):
    code = unlink_run(["/nonexistent_file_xyz_unlink"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "unlink:" in err


def test_unlink_success(capfd):
    with tempfile.NamedTemporaryFile(delete=False) as f:
        path = f.name
        f.write(b"test data")

    assert os.path.exists(path)
    code = unlink_run([path])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""
    assert not os.path.exists(path)
