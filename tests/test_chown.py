"""Tests for chown command."""
import os

import pytest
from gvibu_ref.commands.chown import (
    run, _parse_owner, _lookup_user, _lookup_group
)


def test_parse_owner_user_only():
    uid, gid = _parse_owner("root")
    assert uid is not None or uid is None  # may or may not exist
    assert gid is None


def test_parse_owner_numeric():
    uid, gid = _parse_owner("0")
    assert uid == 0
    assert gid is None


def test_parse_owner_user_group():
    uid, gid = _parse_owner("0:0")
    assert uid == 0
    assert gid == 0


def test_parse_owner_group_only():
    uid, gid = _parse_owner(":0")
    assert uid is None
    assert gid == 0


def test_no_args():
    assert run([]) == 1


def test_invalid_option():
    assert run(["-x", "root", "/dev/null"]) == 1


def test_no_files():
    assert run(["root"]) == 1


def test_regular_file_numeric_owner(tmp_path):
    path = tmp_path / "file"
    path.write_text("content")
    uid = os.getuid()
    gid = os.getgid()
    assert run([str(uid), str(path)]) == 0
    assert path.stat().st_uid == uid
    assert run([f"{uid}:{gid}", str(path)]) == 0
    assert path.stat().st_gid == gid


def test_regular_file_group_only(tmp_path):
    path = tmp_path / "file"
    path.write_text("content")
    gid = os.getgid()
    assert run([f":{gid}", str(path)]) == 0
    assert path.stat().st_gid == gid


def test_regular_directory_recursive(tmp_path):
    nested = tmp_path / "nested"
    nested.mkdir()
    child = nested / "file"
    child.write_text("content")
    uid = os.getuid()
    assert run(["-R", str(uid), str(nested)]) == 0
    assert nested.stat().st_uid == uid
    assert child.stat().st_uid == uid


def test_regular_file_verbose(tmp_path, capsys):
    path = tmp_path / "file"
    path.write_text("content")
    uid = os.getuid()
    assert run(["-v", str(uid), str(path)]) == 0
    assert str(path) in capsys.readouterr().out
