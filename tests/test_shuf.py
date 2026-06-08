"""Tests for shuf."""
import os
import tempfile
from gvibu_ref.commands.shuf import run


def test_too_many_args():
    assert run(["a", "b"]) == 1


def test_nonexistent_file():
    assert run(["/nonexistent_shuf_test_xyz"]) == 1


def test_tmpdir():
    with tempfile.NamedTemporaryFile(mode="w", delete=False) as f:
        f.write("a\nb\nc\n")
        f.flush()
        # Just verify it doesn't crash
        # Can't test exact output since it's randomly shuffled
        fname = f.name
    try:
        import subprocess
        result = subprocess.run(
            ["python3", "-c", f"from gvibu_ref.commands.shuf import run; exit(run(['{fname}']))"],
            capture_output=True, text=True
        )
        assert result.returncode == 0
    finally:
        os.unlink(fname)
