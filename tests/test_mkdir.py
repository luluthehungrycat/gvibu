import os
import tempfile

from gvibu_ref.commands.mkdir import run as mkdir_run


def test_mkdir_no_args(capfd):
    code = mkdir_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "mkdir:" in err


def test_mkdir_invalid_option(capfd):
    code = mkdir_run(["-x", "d"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "mkdir:" in err


def test_mkdir_success(capfd):
    with tempfile.TemporaryDirectory() as tmpdir:
        d = os.path.join(tmpdir, "newdir")
        assert not os.path.exists(d)
        code = mkdir_run([d])
        assert code == 0
        out, err = capfd.readouterr()
        assert err == ""
        assert os.path.isdir(d)


def test_mkdir_existing_no_p(capfd):
    with tempfile.TemporaryDirectory() as tmpdir:
        code = mkdir_run([tmpdir])
        assert code == 1
        out, err = capfd.readouterr()
        assert "File exists" in err


def test_mkdir_p_creates_parents(capfd):
    with tempfile.TemporaryDirectory() as tmpdir:
        d = os.path.join(tmpdir, "a", "b", "c")
        code = mkdir_run(["-p", d])
        assert code == 0
        assert os.path.isdir(d)


def test_mkdir_p_existing_ok(capfd):
    with tempfile.TemporaryDirectory() as tmpdir:
        code = mkdir_run(["-p", tmpdir])
        assert code == 0
