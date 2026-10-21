#!/usr/bin/env python3
"""Generate command status table for docs/command-status.md."""

import os
import json


def get_spec_status(command: str) -> str:
    """Check if spec exists for a command."""
    spec_path = f"specs/commands/{command}.md"
    if os.path.exists(spec_path):
        return "Spec complete"
    return "Spec pending"


def get_python_status(command: str) -> str:
    """Check if a Python implementation exists."""
    candidates = (
        f"python-ref/gvibu_ref/commands/{command}.py",
        f"python-ref/gvibu_ref/commands/{command}_cmd.py",
    )
    if any(os.path.exists(path) for path in candidates):
        return "python-ref implementation"
    return "-"


def get_rust_status(command: str) -> str:
    """Check if a Rust implementation exists."""
    candidates = (
        f"rust/src/commands/{command}.rs",
        f"rust/src/commands/{command}_cmd.rs",
    )
    if any(os.path.exists(path) for path in candidates):
        return "rust implementation"
    return "-"


def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(base_dir)
    
    commands = [
        "true", "false", "echo", "pwd", "basename", "dirname", "cat", "wc",
        "head", "yes", "printenv", "sleep", "touch", "seq", "which", "uname",
        "env", "whoami", "link", "unlink", "tee", "mkdir", "rmdir",
        "hostname", "logname", "readlink", "realpath", "uniq", "uptime",
        "id", "who", "kill", "cut", "tr", "mv", "rm", "ln", "chmod", "chown",
        "sort", "grep", "ls", "cp", "printf", "date", "expr", "split",
        "tail", "tac", "fold", "expand", "rev", "comm", "join", "nl", "shuf",
        "sum", "du", "df", "test",
    ]
    
    print("| Command | Spec | Python (gvibu-ref) | Rust (gvibu) | Notes |")
    print("|---------|------|-------------------|--------------|-------|")
    
    for cmd in commands:
        spec = get_spec_status(cmd)
        python = get_python_status(cmd)
        rust = get_rust_status(cmd)
        
        if spec == "Spec pending":
            notes = "Not started"
        elif python != "-" and rust != "-":
            notes = "Complete"
        elif python != "-":
            notes = "Python done"
        elif rust != "-":
            notes = "Rust done"
        else:
            notes = "Not started"
        
        print(f"| {cmd} | {spec} | {python} | {rust} | {notes} |")


if __name__ == "__main__":
    main()
