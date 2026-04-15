# Hand-off: Plan B - Rust GVIBU runtime + End-to-End Linux scaffolding

Date: 2026-04-15
Owner: GVIBU Bot (coding orchestrator)

Overview
- Plan B focuses on Docker-first runtime validation and parallel End-to-End Linux scaffolding, with Rust GVIBU runtime surface enhancements and Python surface already solid.
- Objective: deliver robust Rust runtime surface with library-based testing, End-to-End Linux scaffolding, and updated docs. Ensure OpenCode task synchronization reflects Ralph tasks.

What has been done (high level)
- Rust runtime: added a GVIBU Rust runtime surface with a gvibu route and a runtime version signal (GVIBU_RUST_VERSION). Dockerfile has a healthcheck and updated COPY semantics to be context-robust.
- Rust tests: library-level tests and CLI smoke tests scaffolds added. Library surface via gvibu_rust_lib to allow unit testing without building binaries.
- End-to-End Linux scaffolding: added/extended scripts for initramfs and QEMU scaffolding, defconfig pieces, and README guidance. These are scaffolds meant for later integration with a kernel/toolchain.
- Documentation: updated gvibu-python, gvibu-rust, and gvibu-linux READMEs with usage, tests, and End-to-End steps. Added OpenCode sidebar and opencode-todos.md, and kept the sync script in place.
- OpenCode sync: OpenCode sidebar file (opencode-sidebar.md) and vanilla to-do (opencode-todos.md) now exist and reflect Ralph’s tasks.

What to verify on next session (step-by-step)
- Docker (no-cache) validation
  - docker-compose down
  - docker-compose build --no-cache
  - docker-compose up -d
  - docker-compose ps
  - Validate health: docker inspect --format '{{.State.Health.Status}}' <container>
  - Quick smoke: docker run -e GVIBU_RUST_VERSION=0.2.0 gvibu-ai-lab-gvibu_rust:latest vish gvibu help
  - Quick smoke: docker run -it --rm gvibu-ai-lab-gvibu_python:latest vish gvibu help
- End-to-End Linux scaffolding verification (Plan B)
  - bash gvibu-linux/build_initramfs.sh
  - bash gvibu-linux/qemu_run_py.sh /path/to/vmlinuz /path/to/initramfs.cpio
  - bash gvibu-linux/qemu_run_rs.sh /path/to/vmlinuz /path/to/initramfs.cpio
  - If you have a kernel, run the defconfig/apply steps and boot with QEMU
- Library tests (Rust)
  - cargo test
  - cargo test -p gvibu_rust_lib (if you add more lib tests)
- OpenCode sync checks
  - bash scripts/sync_opencode_todos.sh
  - Verify opencode-sidebar.md and opencode-todos.md reflect Ralph tasks

Next steps for the next session
- Apply the remaining patches for the End-to-End Linux scaffolding if not yet applied, including any required tweaks based on your environment
- Apply any additional Rust unit tests against the lib surface (lib.rs) to get fast feedback in CI
- Confirm image version signaling via GVIBU_RUST_VERSION in both Python and Rust images (and add image-labels if desired)
- Finalize an optional CI script to run cargo test + pytest in one go and report status

Notes
- I’ve kept changes isolated and patch-ready. If you want, I can also include a minimal protobuf or JSON schema for the GVIBU route messages in the Rust library to standardize the protocol, but that’s extra boilerplate for iteration 1.
