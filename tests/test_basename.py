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


def test_basename_no_suffix(capfd):
    code = basename_run(["/usr/local/bin/testfile.txt"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "testfile.txt"
    assert err == ""


def test_basename_suffix_no_match(capfd):
    code = basename_run(["/a/b/file.txt", ".md"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "file.txt"
    assert err == ""


def test_basename_trailing_slash(capfd):
    code = basename_run(["/a/b/c/"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "c"
    assert err == ""


def test_basename_just_name_suffix(capfd):
    code = basename_run(["foo.txt", ".txt"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip() == "foo"
    assert err == ""


def test_basename_suffix_equals_basename(capfd):
    code = basename_run(["foo", "foo"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "\n"
    assert err == ""


def test_basename_root_path(capfd):
    code = basename_run(["/"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "\n"
    assert err == ""
