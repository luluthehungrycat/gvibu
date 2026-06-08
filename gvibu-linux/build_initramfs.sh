#!/usr/bin/env bash
set -euo pipefail
ROOT="/tmp/gvibu_initramfs"
rm -rf "$ROOT"; mkdir -p "$ROOT/bin"

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Build the Rust gvibu binary
echo "Building Rust gvibu..."
(cd "$REPO_DIR/rust" && cargo build --release)

# Copy gvibu binary and its dynamic dependencies into initramfs
cp "$REPO_DIR/rust/target/release/gvibu" "$ROOT/bin/gvibu"
for lib in /lib/x86_64-linux-gnu/libgcc_s.so.1 \
           /lib/x86_64-linux-gnu/libc.so.6 \
           /lib64/ld-linux-x86-64.so.2; do
    if [ -f "$lib" ]; then
        mkdir -p "$ROOT/$(dirname "$lib")"
        cp "$lib" "$ROOT/$lib"
    fi
done

# Use static busybox as /bin/sh
BUSYBOX_SRC="${BUSYBOX:-/tmp/busybox-x86_64}"
if [ -f "$BUSYBOX_SRC" ]; then
    cp "$BUSYBOX_SRC" "$ROOT/bin/busybox"
    # Create essential busybox applet symlinks (done first; gvibu symlinks below take priority)
    for applet in sh ls cat mount tr sed grep xargs test; do
        ln -sf busybox "$ROOT/bin/$applet"
    done
else
    echo "Warning: busybox not found at $BUSYBOX_SRC - /bin/sh will not be available"
fi

# Create gvibu command symlinks (overrides busybox for these commands)
for cmd in true false echo pwd basename dirname cat wc head yes printenv sleep touch seq which uname env whoami link unlink tee mkdir rmdir hostname logname readlink realpath uniq uptime id who kill cut tr mv rm ln chmod chown sort grep ls cp printf date expr split tail tac fold comm join nl shuf sum du df test '['; do
    ln -sf gvibu "$ROOT/bin/$cmd"
done

cat > "$ROOT/init" <<'SCRIPT'
#!/bin/sh
export PATH=/bin
echo '=== gvibu initramfs boot ==='
echo ''

TESTS_PASSED=0
TESTS_FAILED=0

run_test() {
    local desc="$1"
    shift
    if "$@" > /dev/null 2>&1; then
        echo "  PASS: $desc"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo "  FAIL: $desc"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

check_output() {
    local desc="$1"
    local expected="$2"
    shift 2
    local actual
    actual=$("$@" 2>/dev/null)
    if [ "$actual" = "$expected" ]; then
        echo "  PASS: $desc"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo "  FAIL: $desc (expected='$expected' got='$actual')"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

echo '=== Basic commands ==='
gvibu true            && echo '  PASS: gvibu true'            || echo '  FAIL: gvibu true'
gvibu false           && echo '  FAIL: gvibu false'           || echo '  PASS: gvibu false'

echo ''
echo '=== Symlink dispatch ==='
true   && echo '  PASS: true (symlink)'   || echo '  FAIL: true (symlink)'
false  && echo '  FAIL: false (symlink)'  || echo '  PASS: false (symlink)'

echo ''
echo '=== Output commands ==='
check_output 'echo basic'        'hello'       /bin/echo hello
check_output 'echo -n'           'hello'       /bin/echo -n hello
check_output 'pwd'               '/'           /bin/pwd
check_output 'basename simple'   'bar'         /bin/basename /foo/bar
check_output 'basename suffix'   'file'        /bin/basename /path/file.txt .txt
check_output 'dirname'           '/foo'        /bin/dirname /foo/bar
check_output 'cat stdin'         'hi'          sh -c 'echo hi | /bin/cat'
check_output 'yes output'        'y'           sh -c '/bin/yes 2>/dev/null | /bin/head -n 1'

echo ''
echo '=== Counting and filtering ==='
check_output 'wc /dev/null'      '      0       0       0       0 /dev/null'  /bin/wc /dev/null
check_output 'head -n 0'         ''            /bin/head -n 0 /dev/null

echo ''
echo '=== System commands ==='
/bin/printenv PATH > /dev/null 2>&1 && echo '  PASS: printenv PATH' || echo '  FAIL: printenv PATH'
/bin/uname -a > /dev/null 2>&1     && echo '  PASS: uname'         || echo '  FAIL: uname'

echo ''
echo '=== Testing seq ==='
check_output 'seq 3'             '1 2 3'       sh -c 'echo $(/bin/seq 3)'
check_output 'seq 5 10'          '5 6 7 8 9 10' sh -c 'echo $(/bin/seq 5 10)'

echo ''
echo "=== Tests: $TESTS_PASSED passed, $TESTS_FAILED failed ==="

echo ''
echo '=== gvibu initramfs boot complete ==='
echo 'Dropping to shell...'
exec /bin/sh
SCRIPT

chmod +x "$ROOT/init"

cd "$ROOT"
find . -print0 | cpio --null -ov --format=newc > /tmp/initramfs.cpio 2>/dev/null
echo ""
echo "Initramfs created at /tmp/initramfs.cpio ($(du -sh /tmp/initramfs.cpio | cut -f1))"
echo ""
echo "Run with:"
KERNEL="${KERNEL:-/boot/vmlinuz-6.12.90+deb13.1-cloud-amd64}"
echo "  qemu-system-x86_64 -kernel $KERNEL -initrd /tmp/initramfs.cpio -append 'console=ttyS0' -m 512M -nographic"
