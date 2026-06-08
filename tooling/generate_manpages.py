#!/usr/bin/env python3
"""Generate man pages from command specification files.

Reads specs/commands/*.md and generates man/commandname.1.md files.
Uses pandoc (if available) to produce troff man pages as well.

Usage:
    python3 tooling/generate_manpages.py              # Generate all man pages
    python3 tooling/generate_manpages.py --pandoc     # Also convert to troff format
"""

import re
import os
import sys
import subprocess
from pathlib import Path

SPECS_DIR = Path("specs/commands")
MAN_DIR = Path("man")


def parse_spec(filepath: Path) -> dict:
    """Parse a spec markdown file into structured fields."""
    text = filepath.read_text()
    lines = text.split("\n")

    meta = {
        "name": filepath.stem,
        "synopsis": "",
        "description": "",
        "flags": [],
        "exit_codes": "",
    }

    # Extract first heading as name
    for line in lines:
        m = re.match(r"^#\s+(.+)$", line)
        if m:
            meta["name"] = m.group(1).strip()
            break

    # Extract sections
    current_section = None
    for line in lines:
        if line.startswith("## Synopsis"):
            current_section = "synopsis"
            continue
        elif line.startswith("## Description"):
            current_section = "description"
            continue
        elif line.startswith("## Options") or line.startswith("## Flags"):
            current_section = "flags"
            continue
        elif line.startswith("## Exit"):
            current_section = "exit_codes"
            continue
        elif line.startswith("## "):
            current_section = None

        if current_section == "synopsis":
            stripped = line.strip()
            if stripped and not stripped.startswith("`"):
                meta["synopsis"] += " " + stripped
            elif stripped.startswith("`"):
                meta["synopsis"] = stripped.strip("`")

        elif current_section == "description":
            stripped = line.strip()
            if stripped:
                meta["description"] += " " + stripped

        elif current_section == "flags":
            if re.match(r"^[*-]", line.strip()):
                meta["flags"].append(line.strip())

        elif current_section == "exit_codes":
            stripped = line.strip()
            if stripped:
                meta["exit_codes"] += "\n" + stripped

    return meta


def generate_manpage(meta: dict) -> str:
    """Generate a man page in markdown format."""
    name = meta["name"]
    synopsis = meta["synopsis"] or f"gvibu {name} [OPTIONS] [ARGS...]"
    desc = meta["description"].strip() if meta["description"] else "No description available."
    if desc.startswith("-->"):
        desc = desc.split("-->", 1)[-1].strip()

    lines = []
    lines.append(f"# {name}(1) — gvibu")
    lines.append("")
    lines.append(f"## NAME")
    lines.append("")
    lines.append(f"**{name}** — {desc.split('.')[0] if '.' in desc else desc}")
    lines.append("")
    lines.append(f"## SYNOPSIS")
    lines.append("")
    lines.append(f"`{synopsis}`")
    lines.append("")
    lines.append(f"## DESCRIPTION")
    lines.append("")
    lines.append(desc)
    lines.append("")

    if meta["flags"]:
        lines.append("## OPTIONS")
        lines.append("")
        for flag in meta["flags"]:
            lines.append(f"- {flag}")
        lines.append("")

    if meta["exit_codes"]:
        lines.append("## EXIT STATUS")
        lines.append("")
        for line in meta["exit_codes"].strip().split("\n"):
            lines.append(line.strip())
        lines.append("")

    lines.append("## SEE ALSO")
    lines.append("")
    lines.append("gvibu(1)")
    lines.append("")

    return "\n".join(lines)


def main():
    os.makedirs(MAN_DIR, exist_ok=True)

    spec_files = sorted(SPECS_DIR.glob("*.md"))
    if not spec_files:
        print(f"No spec files found in {SPECS_DIR}")
        sys.exit(1)

    print(f"Generating man pages for {len(spec_files)} commands...")

    for spec_file in spec_files:
        meta = parse_spec(spec_file)
        man_content = generate_manpage(meta)
        man_path = MAN_DIR / f"{meta['name'].lower()}.1.md"
        man_path.write_text(man_content)
        print(f"  created: {man_path}")

    print(f"\nDone. {len(spec_files)} man pages in {MAN_DIR}/")

    if "--pandoc" in sys.argv:
        print("\nConverting to troff format with pandoc...")
        for md_file in sorted(MAN_DIR.glob("*.1.md")):
            troff_file = md_file.with_suffix("")  # Remove .md → .1
            try:
                subprocess.run(
                    ["pandoc", "-s", "-t", "man", "-o", str(troff_file), str(md_file)],
                    check=True, capture_output=True,
                )
                print(f"  converted: {md_file} → {troff_file.name}")
            except (subprocess.CalledProcessError, FileNotFoundError) as e:
                print(f"  skipped {md_file}: pandoc not available or error")
                break

        print("Done.")


if __name__ == "__main__":
    main()
