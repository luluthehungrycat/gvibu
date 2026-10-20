import gvibu_ref.commands.id as id_command
from gvibu_ref.commands.id import groups_with_primary, run as id_run


def test_groups_include_primary_when_supplementary_groups_are_empty():
    assert groups_with_primary([], 7) == [7]


def test_groups_keep_order_and_do_not_duplicate_primary():
    assert groups_with_primary([3, 7, 5], 7) == [3, 7, 5]
    assert groups_with_primary([3, 5], 7) == [3, 5, 7]


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
    assert str(id_command.os.getegid()) in out.split()


def test_id_real_group_and_name_flags_include_selected_primary(monkeypatch, capfd):
    monkeypatch.setattr(id_command.os, "getgroups", lambda: [])
    monkeypatch.setattr(id_command.os, "getgid", lambda: 12)
    monkeypatch.setattr(id_command.os, "getegid", lambda: 34)
    monkeypatch.setattr(id_command, "gid_to_name", lambda gid: f"group-{gid}")

    assert id_run(["-rG"]) == 0
    assert capfd.readouterr().out.strip() == "12"
    assert id_run(["-nG"]) == 0
    assert capfd.readouterr().out.strip() == "group-34"
