from gvibu_ref.commands.head import run as head_run


def test_head_dev_null(capfd):
    code = head_run(["/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_head_dev_null_n_0(capfd):
    code = head_run(["-n", "0", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_head_nonexistent_file(capfd):
    code = head_run(["nonexistent_file_xyz"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "nonexistent_file_xyz" in err


def test_head_invalid_n_arg(capfd):
    code = head_run(["-n", "notanumber", "/dev/null"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "invalid" in err.lower()


def test_head_negative_n(capfd):
    code = head_run(["-n", "-5", "/dev/null"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""


def test_head_missing_n_arg(capfd):
    code = head_run(["-n"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "requires" in err.lower()


def test_head_invalid_option(capfd):
    code = head_run(["-x", "/dev/null"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "invalid" in err.lower()


def test_head_multi_file(capfd):
    code = head_run(["-n", "1", "/dev/null", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert "==> /dev/null <==" in out
    assert err == ""


def test_head_quiet_multi_file(capfd):
    code = head_run(["-q", "/dev/null", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""
