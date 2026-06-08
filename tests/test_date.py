"""Tests for date command."""

from gvibu_ref.commands import date


def test_invalid_option(capsys):
    ret = date.run(["-x"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_extra_operand(capsys):
    ret = date.run(["foo"])
    out, err = capsys.readouterr()
    assert ret == 1
    assert err != ""


def test_default_format(capsys):
    ret = date.run([])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert "2026" in out


def test_utc_flag(capsys):
    ret = date.run(["-u"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""


def test_custom_format(capsys):
    ret = date.run(["+%Y-%m-%d"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    parts = out.strip().split("-")
    assert len(parts) == 3
    assert len(parts[0]) == 4


def test_rfc2822(capsys):
    ret = date.run(["-R"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert out.strip().endswith(" +0000") or "UTC" in out


def test_iso8601(capsys):
    ret = date.run(["-I"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert len(out.strip()) == 10


def test_epoch_seconds(capsys):
    ret = date.run(["+%s"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    ts = out.strip()
    assert ts.isdigit()
    assert len(ts) >= 10


def test_format_custom():
    import datetime
    dt = datetime.datetime(2026, 6, 8, 12, 34, 56)
    assert date.format_custom(dt, "%Y-%m-%d") == "2026-06-08"
    assert date.format_custom(dt, "%H:%M:%S") == "12:34:56"
    assert date.format_custom(dt, "%A") == "Monday"
    assert date.format_custom(dt, "%a") == "Mon"
    assert date.format_custom(dt, "%B") == "June"
    assert date.format_custom(dt, "%b") == "Jun"
    assert date.format_custom(dt, "%%") == "%"
    assert date.format_custom(dt, "%j") == "159"


def test_format_12hour(capsys):
    ret = date.run(["+%I:%M:%S %p"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert ":" in out


def test_format_date_time(capsys):
    ret = date.run(["+%F %T"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert " " in out.strip()


def test_format_r(capsys):
    ret = date.run(["+%r"])
    out, err = capsys.readouterr()
    assert ret == 0
    assert err == ""
    assert ":" in out
