#!/usr/bin/env bash
set -euo pipefail
ROOT="/tmp/gvibu_initramfs"
rm -rf "$ROOT"; mkdir -p "$ROOT/bin" "$ROOT/initramfs"

cat > "$ROOT/init" <<'EOF'
#!/bin/sh
echo 'GVIBU initramfs: GVIBU startup placeholder'
exec /bin/sh
EOF
chmod +x "$ROOT/init"

# Minimal /bin/sh available (via busybox if present)
if [ -d /bin ]; then
        cp -a /bin/sh "$ROOT/bin/sh" 2>/dev/null || true
fi

cd "$ROOT"
find . -print0 | cpio --null -ov --format=newc > /tmp/initramfs.cpio
echo "/tmp/gvibu_initramfs/initramfs.cpio created"
