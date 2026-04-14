#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(pwd)"
FLOWLOOM_ROOT="$(realpath "/../flowloom-ai-edition")"

IMAGE_NAME="opencode-lab"
CONTAINER_NAME="opencode-gvibu-root"

HOST_STATE_ROOT="$REPO_ROOT/.docker-state/opencode-root"
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
  --mount type=bind,src="$REPO_ROOT",dst=/workspace/salience-core-ai-lab \
  --mount type=bind,src="$HOST_CONFIG_DIR",dst=/root/.config/opencode \
  --mount type=bind,src="$HOST_DATA_DIR",dst=/root/.local/share/opencode \
  --mount type=bind,src="$HOST_CACHE_DIR",dst=/root/.cache/opencode \
  -w /workspace/gvibu-ai-lab \
  -e HOME=/root \
  -e OPENCODE_CONFIG_DIR=/root/.config/opencode \
  -e FLOWLOOM_ROOT=/context/flowloom-ai-edition \
  "$IMAGE_NAME"
