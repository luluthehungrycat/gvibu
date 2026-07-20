import os
import subprocess
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), os.pardir))
VISH_BIN = os.path.join(REPO_ROOT, "vish", "target", "debug", "vish")


def _resolve_vish():
    """Return path to the Rust vish binary, building it if necessary."""
    if os.path.exists(VISH_BIN):
        return VISH_BIN
    manifest = os.path.join(REPO_ROOT, "vish", "Cargo.toml")
    subprocess.run(
        ["cargo", "build", "--manifest-path", manifest],
        check=True,
        cwd=REPO_ROOT,
    )
    return VISH_BIN


def run(args, stdin=None):
    cmd = [_resolve_vish()] + list(args)
    result = subprocess.run(
        cmd,
        stdin=subprocess.PIPE if stdin is not None else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        input=stdin,
    )
    return result.stdout, result.stderr, result.returncode


def test_help():
    out, err, code = run(["help"])
    assert code == 0
    assert "gvibu" in out
    assert "Built-in commands" in out


def test_version():
    out, err, code = run(["version"])
    assert code == 0
    assert out.startswith("gvibu v")


def test_commands_lists_58():
    out, err, code = run(["commands"])
    assert code == 0
    lines = [ln for ln in out.splitlines() if ln.strip()]
    assert len(lines) == 58


def test_echo_subcommand():
    out, err, code = run(["echo", "hello", "world"])
    assert code == 0
    assert out == "hello world"


def test_true_subcommand():
    out, err, code = run(["true"])
    assert code == 0


def test_false_subcommand():
    out, err, code = run(["false"])
    assert code == 1


def test_unknown_command():
    out, err, code = run(["nonexistent-command-xyz"])
    assert code == 1
    assert "command not found" in err


def test_repl_pipe_echo_exit():
    out, err, code = run([], stdin="echo hello\nexit\n")
    assert code == 0
    assert "hello" in out


def test_exit_builtin():
    out, err, code = run([], stdin="exit\n")
    assert code == 0
