#!/usr/bin/env python3
"""
Benchmark gvibu: Python ref vs Rust binary.

Measures wall-clock time for each command with various inputs
and reports speedup factors.

Usage:
    python3 tooling/benchmark.py                    # Run all benchmarks
    python3 tooling/benchmark.py --rust path/to/gvibu  # Custom Rust binary
    python3 tooling/benchmark.py --json              # Output as JSON
"""

import subprocess
import sys
import time
import tempfile
import os
import json

PYTHON_CMD = "gvibu-ref"
RUST_CMD = None  # Resolved below

# Benchmarks: list of (name, [args], stdin_data_or_None, repeat_count)
BENCHMARKS = [
    ("true", ["true"], None, 1000),
    ("false", ["false"], None, 500),
    ("echo (1 arg)", ["echo", "hello"], None, 500),
    ("echo (10 args)", ["echo"] + [str(i) for i in range(10)], None, 500),
    ("pwd", ["pwd"], None, 500),
    ("basename", ["basename", "/usr/local/bin/test.txt", ".txt"], None, 500),
    ("dirname", ["dirname", "/usr/local/bin/test.txt"], None, 500),
    ("seq 1000", ["seq", "1000"], None, 100),
    ("seq 10000", ["seq", "10000"], None, 50),
    ("seq -w 1000", ["seq", "-w", "1000"], None, 100),
    ("seq -s ',' 1000", ["seq", "-s", ",", "1000"], None, 100),
    ("wc /dev/null", ["wc", "/dev/null"], None, 500),
    ("head /dev/null", ["head", "/dev/null"], None, 500),
    ("whoami", ["whoami"], None, 500),
    ("printenv", ["printenv"], None, 200),
    ("uname", ["uname", "-a"], None, 200),
    ("tee (stdin)", ["tee"], b"hello\nworld\n" * 100, 200),
]


def find_rust_binary():
    """Find the Rust gvibu binary."""
    candidates = [
        "rust/target/release/gvibu",
        "rust/target/debug/gvibu",
        "./gvibu",
        "../target/release/gvibu",
        "../target/debug/gvibu",
    ]
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    for c in candidates:
        path = os.path.join(repo_root, c)
        if os.path.isfile(path):
            return path
    return None


def time_impl(cmd_base: str, args: list[str], stdin_data: bytes | None, repeat: int) -> tuple[float, int]:
    """Run a command `repeat` times and return (elapsed_seconds, successful_runs)."""
    start = time.perf_counter()
    repeat_actual = 0
    for _ in range(repeat):
        proc = subprocess.run(
            [cmd_base] + args,
            input=stdin_data,
            capture_output=True,
            timeout=30,
        )
        if proc.returncode == 0:
            repeat_actual += 1
        if _ == 0:
            # Warmup — don't count first run
            start = time.perf_counter()
    elapsed = time.perf_counter() - start
    return elapsed, repeat_actual


def run_benchmarks():
    global RUST_CMD
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    RUST_CMD = find_rust_binary()

    # Resolve Python ref
    python_ref = os.path.join(repo_root, "python-ref", "gvibu_ref", "main.py")
    PYTHON_CMD = sys.executable

    if not RUST_CMD:
        print("WARNING: Rust binary not found. Build with: make rust-release")
        print()

    print("=" * 72)
    print("  gvibu Benchmark: Python ref vs Rust")
    print("=" * 72)
    print()

    if RUST_CMD:
        print(f"  Rust binary: {RUST_CMD}")
        # Get version
        try:
            ver = subprocess.run([RUST_CMD, "--version"], capture_output=True, text=True, timeout=5)
            print(f"  Version:     {ver.stdout.strip() or ver.stderr.strip()}")
        except Exception:
            pass
    else:
        print("  Rust binary: NOT AVAILABLE (run 'make rust-release' first)")

    print(f"  Python:      {sys.executable} {sys.version.split()[0]}")
    print()
    print(f"  {'Benchmark':<28} {'Python (s)':>10} {'Rust (s)':>10} {'Speedup':>8} {'Runs':>6}")
    print("  " + "-" * 62)

    results = []

    for name, args, stdin_data, repeat in BENCHMARKS:
        # Python timing
        py_time, _ = time_impl(PYTHON_CMD, ["-m", "gvibu_ref.main"] + args, stdin_data, repeat)

        # Rust timing
        if RUST_CMD:
            rs_time, _ = time_impl(RUST_CMD, args, stdin_data, repeat)
        else:
            rs_time = None

        # Compute speedup
        if rs_time and rs_time > 0:
            speedup = py_time / rs_time
        else:
            speedup = 0.0

        results.append({
            "name": name,
            "python_seconds": round(py_time, 4),
            "rust_seconds": round(rs_time, 4) if rs_time else None,
            "speedup": round(speedup, 2) if rs_time else None,
            "runs": repeat,
        })

        py_str = f"{py_time:.4f}"
        rs_str = f"{rs_time:.4f}" if rs_time else "  N/A  "
        sp_str = f"{speedup:.1f}x" if rs_time else "  -  "

        print(f"  {name:<28} {py_str:>10} {rs_str:>10} {sp_str:>8} {repeat:>6}")

    print()
    print("  (speedup > 1 = Rust faster)")
    print()

    return results


def save_results(results: list[dict], path: str = "benchmark_results.json"):
    with open(path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"  Results saved to {path}")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="gvibu benchmark tool")
    parser.add_argument("--rust", help="Path to Rust gvibu binary")
    parser.add_argument("--json", action="store_true", help="Output as JSON")
    parser.add_argument("--save", help="Save results to JSON file")
    args = parser.parse_args()

    if args.rust:
        if not os.path.isfile(args.rust):
            print(f"Error: Rust binary not found: {args.rust}")
            sys.exit(1)
        RUST_CMD = args.rust

    results = run_benchmarks()

    if args.save:
        save_results(results, args.save)

    if args.json:
        print(json.dumps(results, indent=2))
