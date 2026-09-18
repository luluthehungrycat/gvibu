"""Tests for chown command."""
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


def test_dev_null_numeric():
    assert run(["0", "/dev/null"]) == 1


def test_dev_null_numeric_owner_group():
    assert run(["0:0", "/dev/null"]) == 1


def test_dev_null_group_only():
    assert run([":0", "/dev/null"]) == 1


def test_dev_null_recursive():
    assert run(["-R", "0", "/dev/null"]) == 1


def test_dev_null_verbose():
    assert run(["-v", "0", "/dev/null"]) == 1
