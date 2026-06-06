"""Tests for touch command."""

import os
import tempfile
from gvibu_ref.commands import touch


def test_touch_creates_new_file():
    with tempfile.TemporaryDirectory() as tmpdir:
        path = os.path.join(tmpdir, "newfile")
        assert not os.path.exists(path)
        result = touch.run([path])
        assert result == 0
        assert os.path.exists(path)


def test_touch_updates_existing_file():
    with tempfile.TemporaryDirectory() as tmpdir:
        path = os.path.join(tmpdir, "existing")
        with open(path, "w") as f:
            f.write("content")
        mtime_before = os.path.getmtime(path)
        result = touch.run([path])
        assert result == 0
        mtime_after = os.path.getmtime(path)
        # mtime should be updated (fresher)
        assert mtime_after >= mtime_before


def test_touch_multiple_files():
    with tempfile.TemporaryDirectory() as tmpdir:
        paths = [os.path.join(tmpdir, f"file{i}") for i in range(3)]
        result = touch.run(paths)
        assert result == 0
        for p in paths:
            assert os.path.exists(p)


def test_touch_no_args():
    result = touch.run([])
    assert result == 2


def test_touch_nonexistent_directory():
    result = touch.run(["/nonexistent_dir/file"])
    assert result == 1
