import os
import tempfile

from gvibu_ref.commands.link import run as link_run


def test_link_no_args(capfd):
    code = link_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "link:" in err


def test_link_one_arg(capfd):
    code = link_run(["/dev/null"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "link:" in err


def test_link_nonexistent_source(capfd):
    code = link_run(["/nonexistent_file_xyz", "/tmp/link_test_target"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "link:" in err


def test_link_success(capfd):
    with tempfile.NamedTemporaryFile(delete=False) as f:
        src = f.name
        f.write(b"test data")

    dst = src + "_link"
    try:
        code = link_run([src, dst])
        assert code == 0
        out, err = capfd.readouterr()
        assert out == ""
        assert err == ""
        assert os.path.exists(dst)
        assert os.path.samefile(src, dst)
    finally:
        os.unlink(src)
        if os.path.exists(dst):
            os.unlink(dst)
