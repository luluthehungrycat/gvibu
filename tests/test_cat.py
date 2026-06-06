from gvibu_ref.commands.cat import run as cat_run


def test_cat_dev_null(capfd):
    code = cat_run(["/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_cat_dev_null_twice(capfd):
    code = cat_run(["/dev/null", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_cat_nonexistent_file(capfd):
    code = cat_run(["nonexistent_file_xyz"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "nonexistent_file_xyz" in err
