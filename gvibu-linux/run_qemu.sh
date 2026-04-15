#!/usr/bin/env bash
set -euo pipefail
#
#+ run_qemu.sh: MVP helper to run a linux kernel image with initramfs
#+ Usage: ./run_qemu.sh /path/to/vmlinuz /path/to/initramfs.cpio
KERNEL="${1-}"
INITRAMFS="${2-}"
ROOTFS_DIR="${ROOTFS_DIR:-/tmp/gvibu-root}"
mkdir -p "$ROOTFS_DIR"
if [[ -z "$KERNEL" || -z "$INITRAMFS" ]]; then
  echo "Usage: $(basename "$0") /path/to/vmlinuz /path/to/initramfs.cpio"
  exit 1
fi
echo "QEMU command: qemu-system-x86_64 -kernel \"$KERNEL\" -initrd \"$INITRAMFS\" -append \"root=/dev/ram0 init=/init\" -m 512M -nographic"
echo "(Note: This is a guidance script for MVP. Ensure dependencies are installed in your environment.)"
exit 0
