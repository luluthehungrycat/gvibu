from gvibu_ref.commands.rm import run as rm_run


def test_rm_no_args(capfd):
    code = rm_run([])
    assert code == 1
    out, err = capfd.readouterr()
    assert "rm:" in err


def test_rm_invalid_option(capfd):
    code = rm_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert "rm:" in err


def test_rm_dir_flag_removes_files_and_empty_directories(tmp_path, capfd):
    file = tmp_path / "file"
    empty = tmp_path / "empty"
    nonempty = tmp_path / "nonempty"
    file.write_text("x")
    empty.mkdir()
    nonempty.mkdir()
    (nonempty / "child").write_text("x")

    assert rm_run(["-d", str(file)]) == 0
    assert rm_run(["-d", str(empty)]) == 0
    assert rm_run(["-d", str(nonempty)]) == 1
    out, err = capfd.readouterr()
    assert not file.exists()
    assert not empty.exists()
    assert (nonempty / "child").exists()
    assert "rm:" in err


def test_rm_interactive_yes_and_no(tmp_path, monkeypatch, capfd):
    from gvibu_ref.commands import rm

    keep = tmp_path / "keep"
    remove = tmp_path / "remove"
    keep.write_text("keep")
    remove.write_text("remove")
    answers = iter(("n", "yes"))
    monkeypatch.setattr("builtins.input", lambda prompt: next(answers))

    assert rm.run(["-i", str(keep), str(remove)]) == 0
    assert keep.exists()
    assert not remove.exists()


def test_rm_force_overrides_interactive(tmp_path, monkeypatch):
    from gvibu_ref.commands import rm

    target = tmp_path / "target"
    target.write_text("x")
    monkeypatch.setattr("builtins.input", lambda prompt: (_ for _ in ()).throw(AssertionError("prompted")))
    assert rm.run(["-i", "-f", str(target)]) == 0
    assert not target.exists()


def test_rm_preserve_root_alias_is_rejected(capfd):
    from gvibu_ref.commands import rm
    import os

    if os.name != "posix":
        return
    assert rm.run(["--preserve-root", "/tmp/.."]) == 1
    _, err = capfd.readouterr()
    assert "dangerous" in err


def test_rm_double_dash_allows_dash_prefixed_filename(tmp_path):
    from gvibu_ref.commands import rm

    target = tmp_path / "-x"
    target.write_text("x")
    assert rm.run(["--", str(target)]) == 0
    assert not target.exists()
