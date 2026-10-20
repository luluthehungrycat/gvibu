"""Tests for ls command."""

from gvibu_ref.commands import ls


def test_dev_null(capsys):
    ret = ls.run(["/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out.strip() == "null"
    assert err == ""


def test_invalid_option(capsys):
    ret = ls.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_current_dir(capsys):
    ret = ls.run([])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert len(out) > 0


def test_a_flag(capsys, tmp_path):
    # Create a hidden file
    (tmp_path / ".hidden").write_text("")
    ret = ls.run(["-a", str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert ".hidden" in out


def test_l_flag_dev_null(capsys):
    ret = ls.run(["-l", "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.startswith("-")


def test_bundled_flags(capsys):
    ret = ls.run(["-la", "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.startswith("-")


def test_human_flag(capsys):
    ret = ls.run(["-lh", "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_sort_time(tmp_path, capsys):
    a = tmp_path / "a"
    b = tmp_path / "b"
    a.write_text("")
    b.write_text("")
    ret = ls.run(["-t", str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_reverse(tmp_path, capsys):
    ret = ls.run(["-r", str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_nonexistent(capsys):
    ret = ls.run(["/nonexistent_ls_test_xyz"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_multiple_paths(tmp_path, capsys):
    (tmp_path / "subdir").mkdir()
    ret = ls.run([str(tmp_path), "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_almost_all(capsys, tmp_path):
    (tmp_path / ".hidden").write_text("")
    (tmp_path / "visible").write_text("")
    ret = ls.run(["-A", str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert ".hidden" in out
    assert "visible" in out


def test_sort_by_size(tmp_path, capsys):
    (tmp_path / "small").write_text("x")
    (tmp_path / "large").write_text("x" * 1000)
    ret = ls.run(["-S", str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    lines = out.strip().split("\n")
    assert lines[0] == "large"


def test_feature_flags_directory_itself_indicators_inode_and_color(tmp_path, capsys, monkeypatch):
    from gvibu_ref.commands import ls

    directory = tmp_path / "folder"
    directory.mkdir()
    (directory / "run").write_text("x")
    (directory / "run").chmod(0o755)

    assert ls.run(["-dF", str(directory)]) == 0
    assert capsys.readouterr().out.strip().endswith("folder/")
    assert ls.run(["-F", str(directory)]) == 0
    assert "run*" in capsys.readouterr().out
    assert ls.run(["-i", str(directory)]) == 0
    output = capsys.readouterr().out
    assert output.split()[0].isdigit() and "run" in output

    assert ls.run(["--color=always", str(directory)]) == 0
    assert "\x1b[32mrun\x1b[0m" in capsys.readouterr().out
    monkeypatch.setattr("sys.stdout.isatty", lambda: False)
    assert ls.run(["--color=auto", str(directory)]) == 0
    assert "\x1b[" not in capsys.readouterr().out


def test_ls_double_dash_accepts_dash_prefixed_path(tmp_path, capsys):
    from gvibu_ref.commands import ls

    target = tmp_path / "-x"
    target.write_text("x")
    assert ls.run(["--", str(target)]) == 0
    assert capsys.readouterr().out.strip() == "-x"
