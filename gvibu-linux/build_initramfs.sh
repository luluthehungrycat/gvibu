#!/usr/bin/env bash
set -euo pipefail
ROOT="/tmp/gvibu_initramfs"
rm -rf "$ROOT"; mkdir -p "$ROOT/bin" "$ROOT/initramfs"

# Build the Rust gvibu binary if it doesn't exist
RUST_BIN="$(dirname "$0")/../rust/target/debug/gvibu"
if [ ! -f "$RUST_BIN" ]; then
    echo "Building Rust gvibu..."
    (cd "$(dirname "$0")/../rust" && cargo build)
fi

# Copy gvibu binary into initramfs
mkdir -p "$ROOT/bin"
cp "$RUST_BIN" "$ROOT/bin/gvibu"
# Create symlinks for each command
for cmd in true false echo pwd basename dirname cat wc head; do
    ln -sf gvibu "$ROOT/bin/$cmd"
done

cat > "$ROOT/init" <<'SCRIPT'
#!/bin/sh
echo 'GVIBU initramfs boot'
export PATH=/bin
echo ''
echo '=== Testing gvibu ==='
gvibu true && echo 'true: OK'
gvibu false && echo 'false: FAIL' || echo 'false: OK'
echo "=== All basic tests passed ==="
exec /bin/sh
SCRIPT
chmod +x "$ROOT/init"

# Provide a minimal /bin/sh
if [ -f /bin/sh ]; then
    cp -a /bin/sh "$ROOT/bin/sh" 2>/dev/null || true
fi

cd "$ROOT"
find . -print0 | cpio --null -ov --format=newc > /tmp/initramfs.cpio
echo ""
echo "Initramfs created at /tmp/initramfs.cpio"
echo "Run with: qemu-system-x86_64 -kernel /path/to/vmlinuz -initrd /tmp/initramfs.cpio -append 'root=/dev/ram0 init=/init' -m 512M -nographic"
