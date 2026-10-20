#!/usr/bin/env python3
"""Prepare a self-contained GVIBU VISH initramfs root and newc archive."""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from create_initramfs import create_archive


def _ignore_python_cache(_directory: str, names: list[str]) -> set[str]:
    return {name for name in names if name == "__pycache__" or name.endswith(".pyc")}


def _runtime_paths(python_bin: str) -> dict[str, str]:
    code = "import json,sys,sysconfig; print(json.dumps({'stdlib':sysconfig.get_paths()['stdlib']}))"
    result = subprocess.run(
        [python_bin, "-c", code], check=True, capture_output=True, text=True
    )
    return json.loads(result.stdout)


def _copy_binary_and_dependencies(binary: Path, root: Path, *, strict: bool = True) -> None:
    result = subprocess.run(["ldd", str(binary)], capture_output=True, text=True)
    if result.returncode not in (0, 1):
        raise RuntimeError(f"ldd failed for {binary}: {result.stderr.strip()}")
    if "not a dynamic executable" in result.stdout or "statically linked" in result.stdout:
        return

    for line in result.stdout.splitlines():
        fields = line.strip().split()
        library = None
        if len(fields) >= 3 and fields[1] == "=>":
            if fields[2] == "not":
                if strict:
                    raise RuntimeError(f"missing runtime dependency for {binary}: {line.strip()}")
                continue
            library = fields[2]
        elif fields and fields[0].startswith("/"):
            library = fields[0]
        if not library or not os.path.isfile(library):
            continue
        source = Path(library)
        destination = root / source.relative_to("/")
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, destination, follow_symlinks=True)


def _copy_runtime(binary: Path, root: Path, stdlib: Path | None = None) -> None:
    binary = binary.resolve()
    destination = root / binary.relative_to("/")
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(binary, destination)
    _copy_binary_and_dependencies(binary, root)

    if stdlib is not None:
        stdlib_destination = root / stdlib.relative_to("/")
        shutil.copytree(
            stdlib,
            stdlib_destination,
            symlinks=True,
            dirs_exist_ok=True,
            ignore=shutil.ignore_patterns("site-packages", "__pycache__", "*.pyc"),
        )


def prepare_python(repo: Path, root: Path, python_bin: Path) -> None:
    runtime = _runtime_paths(str(python_bin))
    stdlib = Path(runtime["stdlib"])
    _copy_runtime(python_bin, root, stdlib)

    shutil.copytree(
        repo / "python-ref",
        root / "python-ref",
        symlinks=True,
        ignore=_ignore_python_cache,
    )
    source = (repo / "gvibu-python" / "vish.py").read_text()
    lines = source.splitlines(keepends=True)
    lines[0] = f"#!{python_bin}\n"
    init = root / "init"
    init.write_text("".join(lines))
    init.chmod(0o755)


def prepare_rust(binary: Path, root: Path) -> None:
    destination = root / "bin" / "vish"
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(binary, destination)
    _copy_binary_and_dependencies(binary, root)
    (root / "init").symlink_to("/bin/vish")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("variant", choices=("python", "rust"))
    parser.add_argument("output", type=Path)
    parser.add_argument("--python", default=sys.executable, type=Path)
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--repo", default=Path(__file__).resolve().parent.parent, type=Path)
    args = parser.parse_args()

    with tempfile.TemporaryDirectory(prefix="gvibu-initramfs-") as temporary:
        root = Path(temporary) / "root"
        root.mkdir()
        if args.variant == "python":
            prepare_python(args.repo.resolve(), root, args.python.resolve())
        else:
            if args.binary is None or not args.binary.is_file():
                parser.error("--binary must name a built Rust vish executable")
            prepare_rust(args.binary.resolve(), root)
        create_archive(root, args.output)
    args.output.chmod(0o644)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
