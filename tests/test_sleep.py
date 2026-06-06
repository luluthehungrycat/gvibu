"""Tests for sleep command."""
from gvibu_ref.commands.sleep import run as sleep_run


def test_zero_seconds(capfd):
    """sleep 0 should return immediately."""
    code = sleep_run(["0"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_positive_seconds(capfd):
    """sleep with positive number should block then return 0."""
    import time
    start = time.time()
    code = sleep_run(["1"])
    elapsed = time.time() - start
    assert code == 0
    assert elapsed >= 0.9, f"sleep 1 took {elapsed}s"
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_no_args(capfd):
    """sleep without args should exit 2."""
    code = sleep_run([])
    assert code == 2
    out, err = capfd.readouterr()
    assert out == ""
    assert "usage" in err.lower()


def test_invalid_number(capfd):
    """sleep with non-numeric arg should exit 2."""
    code = sleep_run(["abc"])
    assert code == 2
    out, err = capfd.readouterr()
    assert out == ""
    assert "invalid" in err.lower()


def test_negative_number(capfd):
    """sleep with negative number should exit 1."""
    code = sleep_run(["-5"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "invalid" in err.lower()
