#!/usr/bin/env bash
set -euo pipefail

# Simple tmux-based parallel tester for pwd, echo, and true pilots.
# If tmux is not available, gracefully skip and inform the user.

SESSION="gvibu-build"

if ! command -v tmux >/dev/null 2>&1; then
  echo "tmux not found; skipping tmux-based parallel runs." >&2
  exit 0
fi

tmux has-session -t "$SESSION" 2>/dev/null || {
  tmux new-session -d -s "$SESSION" -n pwd
}

# Ensure a clean start: use separate windows for clarity
tmux rename-window -t "$SESSION":pwd pwd
tmux send-keys -t "$SESSION":pwd "pytest -q tests/test_pwd.py" C-m

tmux new-window -t "$SESSION" -n echo
tmux send-keys -t "$SESSION":echo "pytest -q tests/test_echo.py" C-m

tmux new-window -t "$SESSION" -n true
tmux send-keys -t "$SESSION":true "pytest -q tests/test_true.py" C-m

echo "Tmux windows launched. Attach with: tmux attach -t $SESSION"
tmux select-window -t "$SESSION":pwd
tmux attach -t "$SESSION"
