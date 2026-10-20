"""Regression tests for the Python VISH command dispatcher."""
import os
import subprocess
import sys


REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
VISH = os.path.join(REPO_ROOT, "gvibu-python", "vish.py")


def run_vish(*args: str, stdin: str | None = None):
    return subprocess.run(
        [sys.executable, VISH, *args],
        input=stdin,
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
    )


def test_commands_lists_modules_outside_package_all():
    result = run_vish("commands")
    assert result.returncode == 0
    assert {"cat", "cut", "wc", "which", "env", "test", "["}.issubset(
        set(result.stdout.splitlines())
    )
    assert not {"env_cmd", "test_cmd", "which_cmd"}.intersection(result.stdout.splitlines())


def test_dispatches_command_with_stdin():
    result = run_vish("wc", "-l", stdin="one\ntwo\n")
    assert result.returncode == 0
    assert result.stdout.split() == ["2"]
    assert result.stderr == ""


def test_command_aliases_are_dispatchable():
    assert run_vish("which", "python3").returncode == 0
    assert run_vish("env", "-i", "true").returncode == 0
    assert run_vish("test", "-n", "x").returncode == 0
    assert run_vish("[", "-n", "x", "]").returncode == 0


def test_repl_exit_and_quit_stop_processing_later_commands():
    for builtin in ("exit", "quit"):
        result = run_vish(stdin=f"{builtin}\necho should-not-run\n")
        assert result.returncode == 0
        assert "should-not-run" not in result.stdout


def test_repl_help_then_eof_exits_cleanly():
    result = run_vish(stdin="help\n")
    assert result.returncode == 0
    assert "Built-in commands:" in result.stdout


def test_repl_without_arguments_exits_on_empty_eof():
    result = run_vish(stdin="")
    assert result.returncode == 0
    assert result.stdout == ""
    assert result.stderr == ""


def test_unknown_command_reports_error_status():
    result = run_vish("does-not-exist")
    assert result.returncode == 1
    assert "command not found" in result.stderr
