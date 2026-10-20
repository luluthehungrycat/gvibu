"""Tests for du command."""

from gvibu_ref.commands import du


def test_dev_null(capsys):
    ret = du.run(["/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out == "0\t/dev/null\n"
    assert err == ""


def test_human_readable_dev_null(capsys):
    ret = du.run(["-h", "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out == "0\t/dev/null\n"
    assert err == ""


def test_summary_dev_null(capsys):
    ret = du.run(["-s", "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out == "0\t/dev/null\n"
    assert err == ""


def test_invalid_option(capsys):
    ret = du.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_nonexistent_file(capsys):
    ret = du.run(["/nonexistent_du_test_xyz"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert out == ""
    assert err != ""


def test_bundled_flags(capsys):
    ret = du.run(["-hs", "/dev/null"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert out == "0\t/dev/null\n"
    assert err == ""


def test_max_depth_limits_descendants(tmp_path, capsys):
    nested = tmp_path / "a" / "b"
    nested.mkdir(parents=True)
    (nested / "file.txt").write_text("payload")

    ret = du.run(["--max-depth=1", str(tmp_path)])
    out, err = capsys.readouterr()

    assert ret == 0
    assert err == ""
    assert str(tmp_path / "a") in out
    assert "file.txt" not in out

    full_code = du.run([str(tmp_path)])
    full_out, full_err = capsys.readouterr()
    root_only_code = du.run(["--max-depth=0", str(tmp_path)])
    root_only_out, root_only_err = capsys.readouterr()

    def root_total(output):
        return next(line.split("\t", 1)[0] for line in output.splitlines() if line.endswith(str(tmp_path)))

    assert full_code == root_only_code == 0
    assert full_err == root_only_err == ""
    assert root_total(full_out) == root_total(root_only_out)


def test_no_args(capsys):
    ret = du.run([])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_directory_recursive(tmp_path, capsys):
    d = tmp_path / "subdir"
    d.mkdir()
    f = d / "file.txt"
    f.write_text("hello world")
    ret = du.run([str(tmp_path)])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    lines = out.strip().split("\n")
    assert len(lines) >= 2  # subdir/file + subdir + root
    subdir_line = [l for l in lines if str(d) in l]
    assert len(subdir_line) >= 1  # subdir total


def test_human_size_bytes():
    assert du._human_size(500) == "500"


def test_human_size_kb():
    assert du._human_size(2048) == "2.0K"


def test_human_size_mb():
    assert du._human_size(1048576 * 2) == "2.0M"
