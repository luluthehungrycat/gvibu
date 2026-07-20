#!/usr/bin/env bash
# gvibu-linux/qemu_run_py.sh - Boot the Python vish in a QEMU VM.
#
# Validates prerequisites, builds a Python vish-specific initramfs (containing
# the host python3 interpreter, gvibu-python/vish.py, and dynamic libraries),
# and execs gvibu-linux/run_qemu.sh to perform the actual QEMU launch.
#
# Usage: ./gvibu-linux/qemu_run_py.sh <kernel-path> [initramfs-path]
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
KERNEL_PATH="${1-}"
INITRAMFS_PATH="${2-/tmp/gvibu_py_initramfs.cpio}"

if [[ -z "$KERNEL_PATH" ]]; then
    cat >&2 <<EOF
Error: missing kernel path.
Usage: $(basename "$0") <kernel-path> [initramfs-path]
  <kernel-path>      Path to a Linux kernel image (e.g. /boot/vmlinuz-\$(uname -r))
  [initramfs-path]   Path to a prebuilt initramfs (default: $INITRAMFS_PATH)
                     If the file does not exist, one is built automatically.
EOF
    exit 1
fi

if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
    cat >&2 <<EOF
Error: qemu-system-x86_64 not found in PATH.
Install QEMU for your distribution, e.g.:
  Debian/Ubuntu : sudo apt-get install qemu-system-x86
  Fedora/RHEL   : sudo dnf install qemu-system-x86
  Arch Linux    : sudo pacman -S qemu-system-x86
  macOS         : brew install qemu
EOF
    exit 1
fi

if [[ ! -f "$KERNEL_PATH" ]]; then
    cat >&2 <<EOF
Error: kernel image not found: $KERNEL_PATH
Provide a valid kernel image path, e.g.:
  /boot/vmlinuz-\$(uname -r)
  $REPO_DIR/linux/arch/x86/boot/bzImage
EOF
    exit 1
fi

if [[ ! -f "$REPO_DIR/gvibu-python/vish.py" ]]; then
    echo "Error: gvibu-python/vish.py not found at $REPO_DIR/gvibu-python/vish.py" >&2
    exit 1
fi

# Build the variant initramfs if it's missing or stale.
ROOT="/tmp/gvibu_py_initramfs_root"
SOURCE_VISH="$REPO_DIR/gvibu-python/vish.py"
SCRIPT_PATH="$REPO_DIR/gvibu-linux/qemu_run_py.sh"
needs_build=0
if [[ ! -f "$INITRAMFS_PATH" ]]; then
    needs_build=1
elif [[ "$SOURCE_VISH" -nt "$INITRAMFS_PATH" ]]; then
    needs_build=1
elif [[ "$SCRIPT_PATH" -nt "$INITRAMFS_PATH" ]]; then
    needs_build=1
fi

if [[ "$needs_build" -eq 1 ]]; then
    echo "Building Python vish initramfs at $INITRAMFS_PATH..."

    if ! command -v python3 >/dev/null 2>&1; then
        echo "Error: python3 not found in PATH." >&2
        exit 1
    fi
    PYTHON_BIN="$(command -v python3)"

    # Produce a base initramfs with the existing helper, then augment it.
    "$REPO_DIR/gvibu-linux/build_initramfs.sh"

    rm -rf "$ROOT"
    mkdir -p "$ROOT"
    (cd "$ROOT" && cpio -idm < /tmp/initramfs.cpio >/dev/null 2>&1) || true

    # Install python3 and its dynamic libraries.
    mkdir -p "$ROOT/bin"
    cp "$PYTHON_BIN" "$ROOT/bin/python3"
    chmod +x "$ROOT/bin/python3"

    PYTHON_LIB_DIR="$(dirname "$(ldd "$PYTHON_BIN" | awk '/=> \// {print $3; exit}')")"
    mkdir -p "$ROOT$PYTHON_LIB_DIR"
    while IFS= read -r lib; do
        [[ -z "$lib" || "$lib" == "not" || "$lib" == "a" || "$lib" == "dynamic" ]] && continue
        [[ "$lib" == *"=>"* ]] && lib="${lib##*=> }"
        lib="${lib%% *}"
        [[ -z "$lib" || "$lib" == "linux-vdso.so.1" ]] && continue
        target_dir="$(dirname "$lib")"
        mkdir -p "$ROOT$target_dir"
        cp "$lib" "$ROOT$lib"
    done < <(ldd "$PYTHON_BIN")

    # Install the vish.py script as /bin/vish.py.
    mkdir -p "$ROOT/bin"
    cp "$SOURCE_VISH" "$ROOT/bin/vish.py"
    chmod +x "$ROOT/bin/vish.py"

    # Repack.
    (cd "$ROOT" && find . -print0 | cpio --null -ov --format=newc > "$INITRAMFS_PATH" 2>/dev/null)
    echo "Initramfs created at $INITRAMFS_PATH ($(du -sh "$INITRAMFS_PATH" | cut -f1))"
fi

exec "$REPO_DIR/gvibu-linux/run_qemu.sh" "$KERNEL_PATH" "$INITRAMFS_PATH"
