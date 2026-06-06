"""Tests for env command."""

from gvibu_ref.commands import env_cmd


def test_env_print(capsys):
    result = env_cmd.run([])
    out, _ = capsys.readouterr()
    assert result == 0
    assert len(out) > 0
    assert "=" in out


def test_env_var_assign(capsys):
    result = env_cmd.run(["TEST_GVIBU=hello"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert "TEST_GVIBU=hello" in out


def test_env_unset(capsys):
    result = env_cmd.run(["-u", "PATH"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert "PATH=" not in out


def test_env_ignore(capsys):
    result = env_cmd.run(["-i"])
    out, _ = capsys.readouterr()
    assert result == 0
    assert out == ""


def test_env_run_command():
    """Subprocess output goes to real stdout, not capsys. Check exit code only."""
    result = env_cmd.run(["echo", "hello"])
    assert result == 0


def test_env_run_command_with_var():
    """Subprocess output goes to real stdout, not capsys. Check exit code only."""
    result = env_cmd.run(["TEST_GVIBU=hello", "sh", "-c", "echo $TEST_GVIBU"])
    assert result == 0


def test_env_command_not_found():
    result = env_cmd.run(["nonexistent_cmd_xyz"])
    assert result == 127


def test_env_no_args():
    """Smoke test for environment printing — just ensure no crash."""
    result = env_cmd.run([])
    assert result == 0
