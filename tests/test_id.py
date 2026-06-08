from gvibu_ref.commands.id import run as id_run


def test_id_no_args(capfd):
    code = id_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert "uid=" in out
    assert "gid=" in out


def test_id_invalid_option(capfd):
    code = id_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "id:" in err


def test_id_u_flag(capfd):
    code = id_run(["-u"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip().isdigit()


def test_id_g_flag(capfd):
    code = id_run(["-g"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out.strip().isdigit()


def test_id_G_flag(capfd):
    code = id_run(["-G"])
    assert code == 0
    out, err = capfd.readouterr()
    assert len(out.strip()) > 0
