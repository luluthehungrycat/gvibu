# OpenCode Handoff — Plan B (Docker-first, End-to-End Linux)

Overview
- This document captures the Plan B approach for GVIBU: Docker-first runtime validation with no-cache rebuilds, parallel End-to-End Linux scaffolding, and a Rust runtime surface with a library testing path. It also documents the OpenCode-to-Ralph synchronization and the handoff strategy for the next agent session.

Context
- You already kicked off Docker-first work and parallel End-to-End Linux scaffolding. The Rust runtime surface now includes a library path (gvibu_rust_lib) for unit testing without requiring a binary build, plus a runtime version signal via GVIBU_RUST_VERSION.
- The OpenCode sidebar is synchronized with Ralph's to-dos (opencode-sidebar.md) and opencode-todos.md is kept in-sync with a sync script.

What changed (high level)
- Rust runtime
  - Added library surface (gvibu_rust_lib) and runtime version signaling via GVIBU_RUST_VERSION at runtime
  - Implemented route(cmd, sub) with gvibu help/init/status/run/version and a runtime version path
  - Added lib.rs for library surface and unit tests exercising route and library boundaries
  - Dockerfile healthcheck to validate runtime viability
- End-to-End Linux scaffolding
  - build_initramfs.sh extended to generate a minimal initramfs with GVIBU tooling
  - qemu_run_py.sh and qemu_run_rs.sh extended to produce concrete QEMU command scaffolds for Python and Rust paths
  - kernel_defconfig/gvibu_defconfig refined; apply_defconfig.sh added to apply defconfig to a kernel tree
  - gvibu-linux/README.md updated with End-to-End steps and guidance
- OpenCode sync
  - opencode-sidebar.md updated to reflect Plan B progress and new subfeatures
  - opencode-todos.md kept as a mirror; a separate script maintains alignment
- Documentation synchronization
  - gvibu-python/README.md, gvibu-rust/README.md, and gvibu-linux/README.md updated to reflect the current state and how to verify runtime and end-to-end scaffolds

What you should verify in your environment
- Docker-first checks
  - docker-compose down
  - docker-compose build --no-cache
  - docker-compose up -d
  - docker-compose ps and docker inspect to verify health (healthy expected)
  - Quick runs:
    - docker run -e GVIBU_RUST_VERSION=0.2.0 gvibu-ai-lab-gvibu_rust:latest vish gvibu help
    - docker run gvibu-ai-lab-gvibu_python:latest vish gvibu help
- End-to-End Linux scaffolding (Plan B)
  - bash gvibu-linux/build_initramfs.sh
  - bash gvibu-linux/qemu_run_py.sh /path/to/vmlinuz /path/to/initramfs.cpio
  - bash gvibu-linux/qemu_run_rs.sh /path/to/vmlinuz /path/to/initramfs.cpio
  - Kernel defconfig: patch/apply and boot using QEMU following the End-to-End guide

Next steps for the next session
- Push a final patch set that finishes all remaining Rust runtime patches (including lib-level tests) and End-to-End Linux scaffolding scripts
- Add an image-labeling scheme for GVIBU_VERSION in Dockerfiles to make versioning explicit in image metadata
- Add a CI script to run cargo test + pytest in one go and summarize results
- Finalize the handoff with a detailed PR body for the repo

Assumptions and risks
- The patches assume a Docker-enabled environment with no--cache builds allowed; some CI environments may restrict daemon access and require alternative commands
- End-to-End Linux scaffolding is a scaffold; a real kernel/toolchain is required to fully boot via QEMU

Contacts / Ownership
- Primary owner: GVIBU Bot (internal orchestrator)
- Next handoff owner: to-be-determined in the next session (documentation included for clarity)
