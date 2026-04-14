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
    """Check if Python implementation exists."""
    cmd_path = f"python-ref/gvibu_ref/commands/{command}.py"
    if os.path.exists(cmd_path):
        return f"python-ref implementation"
    return "-"


def get_rust_status(command: str) -> str:
    """Check if Rust implementation exists."""
    cmd_path = f"rust/src/commands/{command}_cmd.rs"
    if os.path.exists(cmd_path):
        return f"rust implementation"
    return "-"


def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(base_dir)
    
    commands = [
        "true", "false", "echo", "pwd",
        "basename", "dirname", "cat", "head", "wc"
    ]
    
    print("| Command | Spec | Python (gvibu-ref) | Rust (gvibu) | Notes |")
    print("|---------|------|-------------------|--------------|-------|")
    
    for cmd in commands:
        spec = get_spec_status(cmd)
        python = get_python_status(cmd)
        rust = get_rust_status(cmd)
        
        if not os.path.exists(f"specs/commands/{cmd}.md") and cmd not in ["basename", "dirname", "cat", "head", "wc"]:
            notes = "Not started"
        elif cmd in ["basename", "dirname", "cat", "head", "wc"]:
            notes = "Scaffolded"
        elif python != "-" and rust != "-":
            notes = "Complete"
        elif python != "-":
            notes = "Python done"
        else:
            notes = "Not started"
        
        print(f"| {cmd} | {spec} | {python} | {rust} | {notes} |")


if __name__ == "__main__":
    main()
