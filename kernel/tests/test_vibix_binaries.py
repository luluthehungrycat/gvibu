#!/usr/bin/env python3
"""
Test harness for VIBIX flat binaries.

Validates structure, size constraints, string tables, and optionally
runs integration tests via the VIBIX kernel in QEMU.

Usage:
    python3 kernel/tests/test_vibix_binaries.py                     # structural checks only
    python3 kernel/tests/test_vibix_binaries.py --vibix-repo /path  # + QEMU integration
    python3 kernel/tests/test_vibix_binaries.py --vibix-repo /path --build  # build + test
"""
import argparse
import os
import struct
import subprocess
import sys
import shutil

KERNEL_DIR = os.path.join(os.path.dirname(__file__), "..")
REPO_ROOT = os.path.join(KERNEL_DIR, "..")

# ── Binary specifications ─────────────────────────────────────────────────────
# name: (file_stem, expected_flags, expected_strings, max_size, description)
BINARIES = [
    ("user_true",      None,      [],                  64,   "exit(0)"),
    ("user_false",     None,      [],                  64,   "exit(1)"),
    ("user_yes",       None,      ["y\n"],             256,  "infinite 'y' loop"),
    ("user_clear",     None,      [b"\x1b[2J", b"\x1b[H"], 128, "ANSI escape clear"),
    ("user_printenv",  None,      [],                  1024, "print env vars"),
    ("user_cat",       None,      [],                  512,  "stdin→stdout copy"),
    ("user_echo_init", {"-n", "-e", "-E", "--"}, ["hello", "world", "no-newline",
     "tab\\there", "plain", "options", "\\0101there"], 4096, "full echo command"),
]


def test_binary(stem):
    """Test a single binary: structure, size, strings."""
    bin_path = os.path.join(KERNEL_DIR, f"{stem}.bin")
    asm_path = os.path.join(KERNEL_DIR, f"{stem}.asm")

    # File existence
    assert os.path.exists(asm_path), f"Missing source: {asm_path}"
    assert os.path.exists(bin_path), (
        f"Missing binary: {bin_path}. Build with: make vibix-{stem[5:]}"
    )

    with open(bin_path, "rb") as f:
        data = f.read()

    # Size check
    assert len(data) <= 4096, (
        f"{stem}.bin too large: {len(data)} bytes (max 4096)"
    )

    # Check starts with valid 64-bit code (no zero bytes at entry)
    # The first bytes should be valid instructions
    non_zero_prefix = 0
    for b in data[:16]:
        if b != 0:
            non_zero_prefix += 1
    assert non_zero_prefix >= 4, (
        f"{stem}.bin: entry point has too many zero bytes (uninitialized data?)"
    )

    # Check entry is at file offset 0 (flat binary, no header)
    assert data[0] != 0x7f or len(data) < 4 or data[1:4] != b"ELF", (
        f"{stem}.bin looks like an ELF, not a flat binary"
    )

    return data, bin_path


def test_echo_especific(bin_path, data):
    """Echo-specific structural checks."""
    # Check for RIP-relative addressing pattern: 48 8d 35 xx xx xx xx
    lea_count = data.count(b"\x48\x8d\x35")
    assert lea_count >= 5, (
        f"Expected >=5 RIP-relative LEA instructions for string references, "
        f"found {lea_count}"
    )

    # Check ORG 0x2000000: first instruction should set RSP near 0x2002000
    # mov rsp, imm32 = bc xx xx xx xx
    assert data[0] == 0xbc, (
        "Expected first instruction to be mov rsp (opcode bc), "
        f"got {data[0]:#04x}"
    )
    rsp_val = struct.unpack_from("<I", data, 1)[0]
    assert rsp_val == 0x2002000, (
        f"Expected RSP init to 0x2002000, got 0x{rsp_val:08x}"
    )


def main():
    parser = argparse.ArgumentParser(description="Test VIBIX flat binaries")
    parser.add_argument("--vibix-repo", help="Path to VIBIX repo for integration tests")
    parser.add_argument(
        "--build", action="store_true", help="Build binaries before testing"
    )
    args = parser.parse_args()

    if args.build:
        subprocess.run(["make", "-C", REPO_ROOT, "vibix-all"], check=True)

    passed = 0
    failed = 0

    for stem, expected_flags, expected_strings, max_size, desc in BINARIES:
        label = stem.removeprefix("user_")
        print(f"  ◇ {label} ({desc})...", end=" ")
        try:
            data, bin_path = test_binary(stem)
            if "echo" in stem:
                test_echo_especific(bin_path, data)
            nbytes = len(data)
            print(f"✅ {nbytes:>4}B", end="")
            if expected_strings:
                found = all(
                    (s.encode() if isinstance(s, str) else s) in data
                    for s in expected_strings
                )
                if found:
                    print("")
                else:
                    print(" ❌ missing strings")
                    failed += 1
                    continue
            passed += 1
            print("")
        except (AssertionError, FileNotFoundError) as e:
            print(f"❌ {e}")
            failed += 1

    print(f"\nResults: {passed} passed, {failed} failed")

    # ── VIBIX integration test ──────────────────────────────────────────────
    if args.vibix_repo:
        print(f"\n── VIBIX integration tests ({args.vibix_repo}) ──")
        vibix_repo = os.path.abspath(args.vibix_repo)
        test_kernel = os.path.join(vibix_repo, "test_kernel.py")
        if os.path.exists(test_kernel):
            result = subprocess.run(
                [sys.executable, test_kernel],
                cwd=vibix_repo,
                capture_output=True,
                text=True,
            )
            print(result.stdout)
            if result.returncode == 0:
                print("✅ VIBIX integration tests passed")
            else:
                print(f"❌ VIBIX integration tests failed (exit {result.returncode})")
                if result.stderr:
                    print("stderr:", result.stderr[:500])
                failed += 1
        else:
            print(f"  ⚠ no test_kernel.py found in {vibix_repo}")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
