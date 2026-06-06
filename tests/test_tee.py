import io
import sys

from gvibu_ref.commands.tee import run as tee_run


def test_tee_stdin_to_stdout(capfd):
    """tee should echo stdin to stdout when no file args."""
    orig_stdin = sys.stdin
    sys.stdin = io.StringIO("hello\nworld\n")
    try:
        code = tee_run([])
        assert code == 0
        out, err = capfd.readouterr()
        assert out == "hello\nworld\n"
        assert err == ""
    finally:
        sys.stdin = orig_stdin


def test_tee_invalid_option(capfd):
    code = tee_run(["-x"])
    assert code == 1
    out, err = capfd.readouterr()
    assert out == ""
    assert "tee:" in err
