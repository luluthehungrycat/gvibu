#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
KERNEL="${1:-/boot/vmlinuz-6.12.90+deb13.1-cloud-amd64}"
INITRAMFS="${2:-/tmp/initramfs.cpio}"

# Build initramfs if it doesn't exist
if [ ! -f "$INITRAMFS" ]; then
    echo "Initramfs not found — building..."
    "$REPO_DIR/gvibu-linux/build_initramfs.sh"
    INITRAMFS="/tmp/initramfs.cpio"
fi

if [ ! -f "$KERNEL" ]; then
    echo "Error: kernel not found at $KERNEL"
    echo "Usage: $(basename "$0") [kernel-path] [initramfs-path]"
    echo ""
    echo "Available kernels:"
    ls /boot/vmlinuz-* 2>/dev/null || echo "  (none found)"
    exit 1
fi

echo "=== gvibu QEMU ==="
echo "Kernel:    $KERNEL"
echo "Initramfs: $INITRAMFS"
echo "Memory:    512M"
echo "Console:   serial (Ctrl+A X to quit)"
echo ""
echo "Starting QEMU..."
qemu-system-x86_64 \
    -kernel "$KERNEL" \
    -initrd "$INITRAMFS" \
    -append "console=ttyS0" \
    -m 512M \
    -nographic
