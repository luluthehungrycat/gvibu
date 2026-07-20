#!/usr/bin/env bash
# gvibu-linux/qemu_run_rs.sh - Boot the Rust vish in a QEMU VM.
#
# Validates prerequisites, builds a Rust vish-specific initramfs (containing
# the vish binary, gvibu library, and dynamic libraries), and execs
# gvibu-linux/run_qemu.sh to perform the actual QEMU launch.
#
# Usage: ./gvibu-linux/qemu_run_rs.sh <kernel-path> [initramfs-path]
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
KERNEL_PATH="${1-}"
INITRAMFS_PATH="${2-/tmp/gvibu_rs_initramfs.cpio}"

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

if [[ ! -f "$REPO_DIR/vish/Cargo.toml" ]]; then
    echo "Error: vish/Cargo.toml not found at $REPO_DIR/vish/Cargo.toml" >&2
    exit 1
fi

# Build the variant initramfs if it's missing or stale.
ROOT="/tmp/gvibu_rs_initramfs_root"
VISH_BIN="$REPO_DIR/vish/target/release/vish"
SOURCE_MAIN="$REPO_DIR/vish/src/main.rs"
SOURCE_CARGO="$REPO_DIR/vish/Cargo.toml"
needs_build=0
if [[ ! -f "$INITRAMFS_PATH" ]]; then
    needs_build=1
elif [[ ! -f "$VISH_BIN" ]]; then
    needs_build=1
elif [[ "$VISH_BIN" -nt "$INITRAMFS_PATH" ]]; then
    needs_build=1
elif [[ "$SOURCE_MAIN" -nt "$INITRAMFS_PATH" ]]; then
    needs_build=1
elif [[ "$SOURCE_CARGO" -nt "$INITRAMFS_PATH" ]]; then
    needs_build=1
fi

if [[ "$needs_build" -eq 1 ]]; then
    echo "Building Rust vish initramfs at $INITRAMFS_PATH..."

    if ! command -v cargo >/dev/null 2>&1; then
        cat >&2 <<EOF
Error: cargo not found in PATH.
Install Rust to build the vish binary, e.g.:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
EOF
        exit 1
    fi

    (cd "$REPO_DIR/vish" && cargo build --release)

    # Produce a base initramfs with the existing helper, then augment it.
    "$REPO_DIR/gvibu-linux/build_initramfs.sh"

    rm -rf "$ROOT"
    mkdir -p "$ROOT"
    (cd "$ROOT" && cpio -idm < /tmp/initramfs.cpio >/dev/null 2>&1) || true

    # Install vish and its dynamic libraries.
    mkdir -p "$ROOT/bin"
    cp "$VISH_BIN" "$ROOT/bin/vish"
    chmod +x "$ROOT/bin/vish"

    while IFS= read -r lib; do
        [[ -z "$lib" ]] && continue
        [[ "$lib" == *"=>"* ]] && lib="${lib##*=> }"
        lib="${lib%% *}"
        [[ -z "$lib" || "$lib" == "linux-vdso.so.1" ]] && continue
        target_dir="$(dirname "$lib")"
        mkdir -p "$ROOT$target_dir"
        cp "$lib" "$ROOT$lib"
    done < <(ldd "$VISH_BIN")

    # Repack.
    (cd "$ROOT" && find . -print0 | cpio --null -ov --format=newc > "$INITRAMFS_PATH" 2>/dev/null)
    echo "Initramfs created at $INITRAMFS_PATH ($(du -sh "$INITRAMFS_PATH" | cut -f1))"
fi

exec "$REPO_DIR/gvibu-linux/run_qemu.sh" "$KERNEL_PATH" "$INITRAMFS_PATH"
