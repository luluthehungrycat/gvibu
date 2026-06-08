from gvibu_ref.commands.wc import run as wc_run


def test_wc_dev_null_default(capfd):
    code = wc_run(["/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0       0       0       0 /dev/null\n"
    assert err == ""


def test_wc_dev_null_lines_only(capfd):
    code = wc_run(["-l", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0 /dev/null\n"
    assert err == ""


def test_wc_dev_null_words_only(capfd):
    code = wc_run(["-w", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0 /dev/null\n"
    assert err == ""


def test_wc_dev_null_bytes_only(capfd):
    code = wc_run(["-c", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0 /dev/null\n"
    assert err == ""


def test_wc_nonexistent_file(capfd):
    code = wc_run(["nonexistent_file_xyz"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "nonexistent_file_xyz" in err


def test_wc_invalid_option(capfd):
    code = wc_run(["-x", "/dev/null"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "invalid" in err.lower()


def test_wc_combined_flags(capfd):
    code = wc_run(["-lw", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    # Only lines and words, no bytes
    assert "      0       0 /dev/null\n" == out
    assert err == ""


def test_wc_chars_only(capfd):
    code = wc_run(["-m", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0 /dev/null\n"
    assert err == ""


def test_wc_all_flags(capfd):
    code = wc_run(["-lwcm", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0       0       0       0 /dev/null\n"
    assert err == ""


def test_wc_max_line_only(capfd):
    code = wc_run(["-L", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0 /dev/null\n"
    assert err == ""


def test_wc_all_flags_with_L(capfd):
    code = wc_run(["-lwcmL", "/dev/null"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "      0       0       0       0       0 /dev/null\n"
    assert err == ""
