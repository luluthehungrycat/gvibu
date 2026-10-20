#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
KERNEL_PATH="${1:-}"
INITRAMFS_PATH="${2:-${TMPDIR:-/tmp}/gvibu_rs_initramfs.cpio}"
QEMU_BIN="${QEMU_BIN:-qemu-system-x86_64}"
TARGET_DIR="${GVIBU_VISH_TARGET_DIR:-${TMPDIR:-/tmp}/gvibu-vish-target}"

if [[ -z "$KERNEL_PATH" ]]; then
    echo "Usage: $(basename "$0") KERNEL [INITRAMFS]" >&2
    exit 2
fi
if [[ ! -f "$KERNEL_PATH" ]]; then
    echo "Error: kernel image not found: $KERNEL_PATH" >&2
    exit 1
fi
if ! command -v "$QEMU_BIN" >/dev/null 2>&1; then
    echo "Error: QEMU executable not found: $QEMU_BIN" >&2
    exit 1
fi

CARGO_TARGET_DIR="$TARGET_DIR" cargo build --release --offline --manifest-path "$REPO_DIR/vish/Cargo.toml"
VISH_BIN="$TARGET_DIR/release/vish"
python3 "$REPO_DIR/gvibu-linux/prepare_vish_initramfs.py" rust "$INITRAMFS_PATH" --binary "$VISH_BIN" --repo "$REPO_DIR"
echo "Booting Rust VISH from $INITRAMFS_PATH"
exec "$QEMU_BIN" \
    -kernel "$KERNEL_PATH" \
    -initrd "$INITRAMFS_PATH" \
    -append "console=ttyS0" \
    -m 512M \
    -nographic
