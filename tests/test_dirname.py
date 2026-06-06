from gvibu_ref.commands.dirname import run as dirname_run


def test_dirname_no_args(capfd):
    code = dirname_run([])
    assert code == 2
    out, err = capfd.readouterr()
    assert out == ""
    assert "usage" in err.lower()


def test_dirname_normal_path(capfd):
    code = dirname_run(["/usr/local/bin"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/usr/local"
    assert err == ""


def test_dirname_deep_path(capfd):
    code = dirname_run(["/a/b/c/file.txt"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/a/b/c"
    assert err == ""


def test_dirname_root_parent(capfd):
    code = dirname_run(["/a"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/"
    assert err == ""


def test_dirname_root_itself(capfd):
    code = dirname_run(["/"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/"
    assert err == ""


def test_dirname_relative_path(capfd):
    code = dirname_run(["a/b/c"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "a/b"
    assert err == ""


def test_dirname_simple_name(capfd):
    code = dirname_run(["a"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "."
    assert err == ""


def test_dirname_trailing_slash(capfd):
    code = dirname_run(["/a/b/c/"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/a/b"
    assert err == ""


def test_dirname_multiple_trailing_slashes(capfd):
    code = dirname_run(["/a/b/c///"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/a/b"
    assert err == ""


def test_dirname_empty_string(capfd):
    code = dirname_run([""])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "."
    assert err == ""


def test_dirname_double_slash_root(capfd):
    code = dirname_run(["//"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "/"
    assert err == ""
