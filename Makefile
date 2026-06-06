.PHONY: all build python-build rust-build test compare python-test rust-test \
        docker-build initramfs qemu run install clean status help

PYTHON := python3
CARGO := $(HOME)/.cargo/bin/cargo

all: python-build rust-build

python-build:
	@echo "Python reference is interpreted - no build needed"

rust-build:
	$(CARGO) build --manifest-path rust/Cargo.toml

rust-release:
	$(CARGO) build --release --manifest-path rust/Cargo.toml

test: compare

compare:
	$(PYTHON) tooling/compare_impls.py

python-test:
	PYTHONPATH=python-ref $(PYTHON) -m pytest tests/ -v

rust-test:
	$(CARGO) test --manifest-path rust/Cargo.toml

docker-build:
	docker build -t gvibu:latest -f Dockerfile .

initramfs: rust-release
	gvibu-linux/build_initramfs.sh

qemu: initramfs
	gvibu-linux/run_qemu.sh

install: rust-release
	cp rust/target/release/gvibu /usr/local/bin/gvibu
	@for cmd in true false echo pwd basename dirname cat wc head yes printenv sleep touch seq which uname env whoami link unlink tee; do \
		ln -sf /usr/local/bin/gvibu "/usr/local/bin/$$cmd"; \
	done
	@echo "Installed gvibu and symlinks to /usr/local/bin"

status:
	$(PYTHON) tooling/generate_command_status.py

clean:
	cd rust && $(CARGO) clean
	rm -rf rust/target

help:
	@echo "gvibu build system"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build both implementations"
	@echo "  python-build - Setup Python reference (no build needed)"
	@echo "  rust-build   - Build Rust implementation (debug)"
	@echo "  rust-release - Build Rust implementation (release)"
	@echo "  test         - Run comparison tests"
	@echo "  compare      - Compare Python and Rust implementations"
	@echo "  python-test  - Run Python unit tests"
	@echo "  rust-test    - Run Rust unit tests"
	@echo "  docker-build - Build Docker container image"
	@echo "  initramfs    - Build QEMU initramfs image"
	@echo "  qemu         - Build initramfs and run in QEMU"
	@echo "  install      - Install gvibu binary and symlinks"
	@echo "  status       - Show command implementation status"
	@echo "  clean        - Clean build artifacts"
