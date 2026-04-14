import os
import sys


def test_true_no_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.true import run as true_run

    code = true_run([])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""


def test_true_with_args(capfd):
    sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "python-ref"))
    from gvibu_ref.commands.true import run as true_run

    code = true_run(["a", "b"])
    assert code == 0
    out, err = capfd.readouterr()
    assert out == ""
    assert err == ""
