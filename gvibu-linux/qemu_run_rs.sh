#!/usr/bin/env bash
set -euo pipefail
# Simple QEMU run helper for Rust GVIBU variant (scaffold)
KERNEL_PATH="${1-}"
INITRAMFS_PATH="${2-}"
if [[ -z "$KERNEL_PATH" || -z "$INITRAMFS_PATH" ]]; then
  echo "Usage: ./gvibu-linux/qemu_run_rs.sh /path/to/vmlinuz /path/to/initramfs.cpio"
  exit 1
fi
echo "qemu-system-x86_64 -kernel \"$KERNEL_PATH\" -initrd \"$INITRAMFS_PATH\" -append \"root=/dev/ram0 init=/init\" -m 512M -nographic"
echo "Note: This is a scaffold for the Rust GVIBU path"
