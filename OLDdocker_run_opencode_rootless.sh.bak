#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(pwd)"
FLOWLOOM_ROOT="$(realpath "../flowloom-ai-edition")"

IMAGE_NAME="salience-core-opencode-lab:nonroot"
CONTAINER_NAME="gvibu-opencode-lab-nonroot"

HOST_STATE_ROOT="$REPO_ROOT/.docker-state/opencode-user"
HOST_CONFIG_DIR="$HOST_STATE_ROOT/config"
HOST_DATA_DIR="$HOST_STATE_ROOT/local-share"
HOST_CACHE_DIR="$HOST_STATE_ROOT/cache"

mkdir -p \
  "$HOST_CONFIG_DIR" \
  "$HOST_DATA_DIR" \
  "$HOST_CACHE_DIR"

docker run -it --rm \
  --name "$CONTAINER_NAME" \
  --network host \
  --mount type=bind,src="$REPO_ROOT",dst=/workspace/gvibu-ai-lab \
  --mount type=bind,src="$HOST_CONFIG_DIR",dst=/home/opencode/.config/opencode \
  --mount type=bind,src="$HOST_DATA_DIR",dst=/home/opencode/.local/share/opencode \
  --mount type=bind,src="$HOST_CACHE_DIR",dst=/home/opencode/.cache/opencode \\
  --mount type=bind,src="$FLOWLOOM_ROOT",dst=/context/flowloom-ai-edition,readonly \
  -w /workspace/gvibu-ai-lab \
  -e HOME=/home/opencode \
  -e OPENCODE_CONFIG_DIR=/home/opencode/.config/opencode \
  -e FLOWLOOM_ROOT=/context/flowloom-ai-edition \
  "$IMAGE_NAME"
