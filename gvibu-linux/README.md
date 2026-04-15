GVIBU/Linux MVP scaffolding
Overview:
- Provides a minimal Linux userland testing path for GVIBU
- MVP builds a vish shell, a qemu run helper, and a docker container with GVIBU tools

How to use:
- Build and run vish locally: ./gvibu-linux/vish help
- Use run_qemu.sh to assemble and run in QEMU with a kernel/initramfs
- Build docker container from gvibu-linux/docker/Dockerfile

Notes:
- This is a scaffold; actual GVIBU integration will require a kernel, initramfs, and proper toolchains

Testing GVIBU/Linux (MVP) testing guide
- Prerequisites: qemu-system-x86_64, a host kernel image (vmlinuz), and an initramfs (initramfs.cpio) with GVIBU tools installed
- QEMU-based test (recommended):
  1) Prepare kernel and initramfs: provide paths to vmlinuz and initramfs.cpio
  2) Run: ./gvibu-linux/run_qemu.sh /path/to/vmlinuz /path/to/initramfs.cpio
  3) Observe output; this scaffold currently prints guidance rather than booting a full GVIBU userland
- Docker-based test:
  1) Build the container: docker build -t gvibu/vish gvibu-linux/docker
 2) Run a shell: docker run -it gvibu/vish
 3) Inside container, run: /gvibu-linux/vish help
- In-code testing:
- In-code testing:
- In-code testing (Python):
  - pytest-based tests for vish-cli Python variant are provided at tests/test_vish_py.py (run from repo root: pytest -q)
- QEMU-based tests:
  - Use gvibu-linux/qemu_run_py.sh and gvibu-linux/qemu_run_rs.sh as scaffolds to run QEMU with provided kernel/initramfs placeholders

### End-to-End Linux scaffolding (Plan B)
- Goal: demonstrate a runnable Linux-userland path using GVIBU tooling in a kernel/initramfs boot flow (scaffold only for now)
- What you get:
  - Minimal initramfs generation script (build_initramfs.sh) that packages GVIBU tooling
  - QEMU command scaffolds for Python and Rust GVIBU paths (qemu_run_py.sh, qemu_run_rs.sh)
  - Starter kernel defconfig (gvibu_defconfig) and a helper (apply_defconfig.sh)
  - Updated End-to-End docs with step-by-step verification
- How to run:
  1) Build a minimal initramfs: ./gvibu-linux/build_initramfs.sh
  2) Print QEMU commands for Python or Rust GVIBU paths:
     - ./gvibu-linux/qemu_run_py.sh /path/to/vmlinuz /path/to/initramfs.cpio
     - ./gvibu-linux/qemu_run_rs.sh /path/to/vmlinuz /path/to/initramfs.cpio
  3) If you have a kernel source tree, apply a defconfig and build:
     - ./gvibu-linux/kernel_defconfig/gvibu_defconfig for the defconfig target
     - ./gvibu-linux/apply_defconfig.sh /path/to/kernel
  4) Boot with QEMU using the generated initramfs for end-to-end flow
  5) Docker-based verification (optional): run the Python and Rust GVIBU containers and verify basic GVIBU help output
  6) Versioning: use GVIBU_RUST_VERSION env var for Rust container to confirm versioning
- Note: This is a scaffold; actual full boot requires a kernel and build toolchain suitable for your environment.
