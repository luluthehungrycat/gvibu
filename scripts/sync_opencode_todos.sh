#!/usr/bin/env bash
set -euo pipefail
SRC=".ralph/ralph-tasks.md"
DST="opencode-todos.md"
if [[ ! -f "$SRC" ]]; then
  echo "Source Ralph tasks not found: $SRC"; exit 1
fi
cat > "$DST" << 'EOS'
# OpenCode Vanilla ToDo - GVIBU
EOS
python3 - <<'PY'
import re
SRC = ".ralph/ralph-tasks.md"
DST = "opencode-todos.md"
with open(SRC, 'r') as f:
    lines = f.read().splitlines()
out = ["# OpenCode Vanilla ToDo - GVIBU"]
for line in lines:
  if line.startswith("- [x] "):
    out.append("- [x] " + line[len("- [x] "):])
  elif line.startswith("- [/]") or line.startswith("- [/] "):
    # Normalize to an empty task (not started)
    rest = line.split(" ", 2)[2]
    out.append("- [ ] " + rest)
  elif line.startswith("- [ ] "):
    out.append(line)
  elif line.strip().startswith('#'):
    # include header lines
    out.append(line)
with open(DST, 'w') as f:
  f.write("\n".join(out))
print("Wrote OpenCode TODOs to " + DST)
PY
echo
echo "OpenCode todos synced to $DST"
