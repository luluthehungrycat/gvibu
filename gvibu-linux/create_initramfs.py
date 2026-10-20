#!/usr/bin/env python3
"""Write an uncompressed ``newc`` initramfs from a prepared root directory."""

import os
import stat
import sys
from pathlib import Path


CPIO_HEADER_SIZE = 110


def _pad4(data: bytes) -> bytes:
    return data + b"\0" * (-len(data) % 4)


def _entry(name: str, path: Path, inode: int) -> bytes:
    name_bytes = name.encode() + b"\0"
    if name in ("dev/console", "dev/null") and not path.exists():
        mode = stat.S_IFCHR | (0o600 if name == "dev/console" else 0o666)
        data = b""
    else:
        info = path.lstat()
        mode = info.st_mode
        if stat.S_ISLNK(mode):
            data = os.readlink(path).encode()
        elif stat.S_ISREG(mode):
            data = path.read_bytes()
        elif stat.S_ISDIR(mode):
            data = b""
        else:
            raise ValueError(f"unsupported initramfs entry: {name}")

    if name == "dev/console":
        rdevmajor, rdevminor = 5, 1
    elif name == "dev/null":
        rdevmajor, rdevminor = 1, 3
    else:
        rdevmajor = rdevminor = 0

    fields = (
        inode,
        mode,
        0,
        0,
        2 if stat.S_ISDIR(mode) else 1,
        0,
        len(data),
        0,
        0,
        rdevmajor,
        rdevminor,
        len(name_bytes),
        0,
    )
    header = b"070701" + b"".join(f"{value:08x}".encode() for value in fields)
    if len(header) != CPIO_HEADER_SIZE:
        raise AssertionError("invalid newc header size")
    # newc's 110-byte header is not word-aligned.  Padding the name by its
    # length alone leaves each payload two bytes out of alignment.
    name_padding = b"\0" * (-(CPIO_HEADER_SIZE + len(name_bytes)) % 4)
    return header + name_bytes + name_padding + _pad4(data)


def create_archive(root: Path, output: Path) -> None:
    root = root.resolve()
    if not root.is_dir():
        raise ValueError(f"initramfs root is not a directory: {root}")

    # Kernels expect these device paths even when the host initramfs source
    # tree has no /dev directory.  CPIO cannot encode a child of a missing
    # directory, so create and include the directory before its devices.
    dev_dir = root / "dev"
    if not dev_dir.exists():
        dev_dir.mkdir()
    if not dev_dir.is_dir():
        raise ValueError(f"initramfs dev path is not a directory: {dev_dir}")

    names = ["."]
    for directory, subdirs, files in os.walk(root, followlinks=False):
        subdirs.sort()
        files.sort()
        relative = Path(directory).relative_to(root)
        names.extend((relative / name).as_posix() for name in subdirs + files)
    names.extend(name for name in ("dev/console", "dev/null") if name not in names)

    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("wb") as archive:
        for inode, name in enumerate(names, start=1):
            path = root if name == "." else root / name
            archive.write(_entry(name, path, inode))
        trailer = b"TRAILER!!!"
        trailer_fields = (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, len(trailer) + 1, 0)
        archive.write(b"070701" + b"".join(f"{v:08x}".encode() for v in trailer_fields))
        trailer_name = trailer + b"\0"
        archive.write(trailer_name)
        archive.write(b"\0" * (-(CPIO_HEADER_SIZE + len(trailer_name)) % 4))
        archive.write(_pad4(b""))


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(f"Usage: {Path(argv[0]).name} ROOT_DIR OUTPUT.cpio", file=sys.stderr)
        return 2
    try:
        create_archive(Path(argv[1]), Path(argv[2]))
    except (OSError, ValueError) as error:
        print(f"initramfs: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
