import pytest
import os
import sys


def test_echo_no_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.echo import run as echo_run

    code = echo_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "\n"
    assert err == ""


def test_echo_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.echo import run as echo_run

    code = echo_run(["hello", "world"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == "hello world\n"
    assert err == ""
