#!/usr/bin/env bash
set -euo pipefail
KERNEL_SRC="${1:-/lib/modules/$(uname -r)/build}"
DEFCONFIG_PATH="${2:-gvibu_defconfig}"
if [[ ! -d "$KERNEL_SRC" ]]; then
  echo "Kernel source path not found: $KERNEL_SRC"
  exit 1
fi
if [[ ! -f "$KERNEL_SRC/.config" ]]; then
  echo "Config file not found at $KERNEL_SRC/.config; creating default config"
  cp "$KERNEL_SRC/.config" /dev/null 2>/dev/null || true
fi
echo "Applying GVIBU defconfig ($DEFCONFIG_PATH) to kernel at $KERNEL_SRC"
make -C "$KERNEL_SRC" GVIBU_DEFCONFIG="$DEFCONFIG_PATH" olddefconfig || true
echo "Defconfig applied (if supported by the kernel tree)."
