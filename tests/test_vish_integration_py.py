import os
import subprocess
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), os.pardir))
VISH_PY = os.path.join(REPO_ROOT, "gvibu-python", "vish.py")
VISH_RS = os.path.join(REPO_ROOT, "vish", "target", "debug", "vish")


def _run_python(args, stdin=None):
    cmd = [sys.executable, VISH_PY] + list(args)
    result = subprocess.run(
        cmd,
        stdin=subprocess.PIPE if stdin is not None else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        input=stdin,
    )
    return result.stdout, result.stderr, result.returncode


def _run_rust(args, stdin=None):
    if not os.path.exists(VISH_RS):
        manifest = os.path.join(REPO_ROOT, "vish", "Cargo.toml")
        subprocess.run(
            ["cargo", "build", "--manifest-path", manifest],
            check=True,
            cwd=REPO_ROOT,
        )
    cmd = [VISH_RS] + list(args)
    result = subprocess.run(
        cmd,
        stdin=subprocess.PIPE if stdin is not None else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        input=stdin,
    )
    return result.stdout, result.stderr, result.returncode


def test_python_help_matches_rust_help():
    py_out, _, py_code = _run_python(["help"])
    rs_out, _, rs_code = _run_rust(["help"])
    assert py_code == 0
    assert rs_code == 0
    assert py_out.strip() == rs_out.strip()


def test_python_echo_matches_rust_echo():
    py_out, _, py_code = _run_python(["echo", "parity"])
    rs_out, _, rs_code = _run_rust(["echo", "parity"])
    assert py_code == 0
    assert rs_code == 0
    assert py_out == rs_out == "parity"


def test_python_true_matches_rust_true():
    _, _, py_code = _run_python(["true"])
    _, _, rs_code = _run_rust(["true"])
    assert py_code == 0
    assert rs_code == 0


def test_python_false_matches_rust_false():
    _, _, py_code = _run_python(["false"])
    _, _, rs_code = _run_rust(["false"])
    assert py_code == 1
    assert rs_code == 1


def test_python_unknown_matches_rust_unknown():
    _, py_err, py_code = _run_python(["nonexistent-command-xyz"])
    _, rs_err, rs_code = _run_rust(["nonexistent-command-xyz"])
    assert py_code == 1
    assert rs_code == 1
    assert "command not found" in py_err
    assert "command not found" in rs_err


def test_python_repl_matches_rust_repl():
    py_out, _, py_code = _run_python([], stdin="echo repl\nexit\n")
    rs_out, _, rs_code = _run_rust([], stdin="echo repl\nexit\n")
    assert py_code == 0
    assert rs_code == 0
    assert "repl" in py_out
    assert "repl" in rs_out
