import importlib.util
import stat
import sys
from pathlib import Path


SCRIPT = Path(__file__).parents[1] / "gvibu-linux" / "create_initramfs.py"
SPEC = importlib.util.spec_from_file_location("create_initramfs", SCRIPT)
create_initramfs = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(create_initramfs)

sys.path.insert(0, str(SCRIPT.parent))
PREPARE_SCRIPT = SCRIPT.with_name("prepare_vish_initramfs.py")
PREPARE_SPEC = importlib.util.spec_from_file_location("prepare_vish_initramfs", PREPARE_SCRIPT)
prepare_vish_initramfs = importlib.util.module_from_spec(PREPARE_SPEC)
PREPARE_SPEC.loader.exec_module(prepare_vish_initramfs)


def _read_newc(path):
    entries = {}
    offset = 0
    data = path.read_bytes()
    while True:
        assert data[offset : offset + 6] == b"070701"
        fields = [int(data[offset + 6 + i * 8 : offset + 14 + i * 8], 16) for i in range(13)]
        mode, size, name_size = fields[1], fields[6], fields[11]
        name_start = offset + 110
        name_end = name_start + name_size
        name = data[name_start : name_end - 1].decode()
        payload_start = (name_end + 3) & ~3
        payload_end = payload_start + size
        assert payload_start % 4 == 0
        if name == "TRAILER!!!":
            return entries
        entries[name] = (mode, data[payload_start:payload_end])
        offset = (payload_end + 3) & ~3


def test_archive_has_aligned_entries_and_missing_device_nodes(tmp_path):
    root = tmp_path / "root"
    root.mkdir()
    (root / "init").write_text("#!/bin/sh\n")
    archive = tmp_path / "initramfs.cpio"

    create_initramfs.create_archive(root, archive)
    entries = _read_newc(archive)

    assert entries["init"][1] == b"#!/bin/sh\n"
    assert stat.S_ISDIR(entries["dev"][0])
    assert stat.S_ISCHR(entries["dev/console"][0])
    assert stat.S_ISCHR(entries["dev/null"][0])
    assert len(entries) == 5


def test_python_runtime_staging_excludes_bytecode_and_cache_dirs(tmp_path):
    cache_dir = tmp_path / "__pycache__"
    cache_dir.mkdir()
    (cache_dir / "module.pyc").write_bytes(b"cache")
    (tmp_path / "module.pyc").write_bytes(b"cache")
    (tmp_path / "module.py").write_text("source")

    names = [path.name for path in tmp_path.iterdir()]
    ignored = prepare_vish_initramfs._ignore_python_cache(str(tmp_path), names)
    assert ignored == {"__pycache__", "module.pyc"}
