#!/usr/bin/env python3
"""Compare implementations of gvibu commands."""

import json
import os
import subprocess
import sys


def run_command(cmd: list[str], stdin: str = "") -> tuple[int, str, str]:
    """Run a command and return (exit_code, stdout, stderr)."""
    try:
        result = subprocess.run(
            cmd,
            input=stdin,
            capture_output=True,
            text=True,
            timeout=10,
        )
        return result.returncode, result.stdout, result.stderr
    except Exception as e:
        return 1, "", str(e)


def load_test_cases(command: str) -> list[dict]:
    """Load test cases for a command from shared-tests."""
    path = f"shared-tests/cases/{command}.json"
    if not os.path.exists(path):
        return []
    with open(path) as f:
        data = json.load(f)
    return data.get("cases", [])


def run_tests_for_command(
    command: str,
    python_cmd: str,
    rust_cmd: str,
) -> dict:
    """Run all test cases for a command against both implementations."""
    cases = load_test_cases(command)
    results = {
        "command": command,
        "total": len(cases),
        "passed": 0,
        "failed": 0,
        "details": [],
    }

    for case in cases:
        name = case.get("name", "unnamed")
        args = case.get("args", [])
        expected_stdout = case.get("stdout", "")
        expected_stderr = case.get("stderr", "")
        expected_exit = case.get("exit_code", 0)

        python_full_cmd = python_cmd + [command] + args
        rust_full_cmd = rust_cmd + [command] + args

        py_code, py_out, py_err = run_command(python_full_cmd)
        rust_code, rust_out, rust_err = run_command(rust_full_cmd)

        py_match = (
            py_out == expected_stdout
            and py_err == expected_stderr
            and py_code == expected_exit
        )
        rust_match = (
            rust_out == expected_stdout
            and rust_err == expected_stderr
            and rust_code == expected_exit
        )

        parity = py_match and rust_match

        if parity:
            results["passed"] += 1
        else:
            results["failed"] += 1

        results["details"].append(
            {
                "name": name,
                "python_match": py_match,
                "rust_match": rust_match,
                "parity": parity,
                "python_output": {
                    "stdout": py_out,
                    "stderr": py_err,
                    "exit_code": py_code,
                },
                "rust_output": {
                    "stdout": rust_out,
                    "stderr": rust_err,
                    "exit_code": rust_code,
                },
                "expected": {
                    "stdout": expected_stdout,
                    "stderr": expected_stderr,
                    "exit_code": expected_exit,
                },
            }
        )

    return results


def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(base_dir)

    python_cmd = ["python3", "python-ref/gvibu_ref/main.py"]
    rust_cmd = ["rust/target/debug/gvibu"]

    commands = ["true", "false", "echo", "pwd", "basename", "dirname"]

    all_results = []

    for cmd in commands:
        print(f"\n=== Testing {cmd} ===")
        results = run_tests_for_command(cmd, python_cmd, rust_cmd)
        all_results.append(results)

        print(
            f"Total: {results['total']}, Passed: {results['passed']}, Failed: {results['failed']}"
        )

        for detail in results["details"]:
            if not detail["parity"]:
                print(f"  FAIL: {detail['name']}")
                if not detail["python_match"]:
                    print(f"    Python: exit={detail['python_output']['exit_code']}")
                if not detail["rust_match"]:
                    print(f"    Rust: exit={detail['rust_output']['exit_code']}")

    total_tests = sum(r["total"] for r in all_results)
    total_passed = sum(r["passed"] for r in all_results)
    total_failed = sum(r["failed"] for r in all_results)

    print(f"\n=== Summary ===")
    print(f"Total: {total_tests}, Passed: {total_passed}, Failed: {total_failed}")

    if total_failed > 0:
        sys.exit(1)
    else:
        print("All tests passed!")
        sys.exit(0)


if __name__ == "__main__":
    main()
