from gvibu_ref.commands.tr import run as tr_run


def test_tr_no_args(capfd):
    code = tr_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "tr:" in err


def test_tr_invalid_option(capfd):
    code = tr_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "tr:" in err


def test_tr_squeeze_accepts_one_set(monkeypatch, capfd):
    monkeypatch.setattr("sys.stdin", type("Input", (), {"buffer": __import__("io").BytesIO(b"a    b   c\\n")})())
    code = tr_run(["-s", " "])
    out, err = capfd.readouterr()
    assert code == 0
    assert out == "a b c\\n"
    assert err == ""
