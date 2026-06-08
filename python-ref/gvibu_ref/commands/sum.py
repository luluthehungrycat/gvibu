"""sum: compute BSD 16-bit checksum and block count."""
import sys


def bsd_checksum(data: bytes) -> int:
    checksum = 0
    for byte in data:
        checksum = ((checksum >> 1) | ((checksum & 1) << 15)) + byte
    return checksum & 0xFFFF


def run(args: list[str]) -> int:
    files = [a for a in args if not a.startswith('-')]
    if not files:
        print("sum: missing operand", file=sys.stderr)
        return 1

    for fname in files:
        try:
            with open(fname, "rb") as f:
                data = f.read()
            cksum = bsd_checksum(data)
            blocks = (len(data) + 1023) // 1024
            print(f"{cksum} {blocks}")
        except FileNotFoundError as e:
            print(f"sum: {fname}: {e}", file=sys.stderr)
            return 1
    return 0
