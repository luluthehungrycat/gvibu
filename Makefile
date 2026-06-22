.PHONY: all build python-build rust-build wasm-build wasm-run wasm-test \
        wasm-browser-build wasm-browser-serve wasm-browser-clean \
        test compare python-test rust-test benchmark \
        docker-build initramfs qemu run install clean status help \
        release release-binary release-wasm release-docker man \
        vibix-all vibix-clean \
        vibix-echo vibix-echocmd vibix-true vibix-false vibix-yes vibix-clear \
        vibix-printenv vibix-cat

PYTHON := python3
CARGO := $(HOME)/.cargo/bin/cargo
NASM  := nasm

all: python-build rust-build

python-build:
	@echo "Python reference is interpreted - no build needed"

rust-build:
	$(CARGO) build --manifest-path rust/Cargo.toml

rust-release:
	$(CARGO) build --release --manifest-path rust/Cargo.toml

wasm-build:
	rustup target add wasm32-wasi && \
	$(CARGO) build --release --target wasm32-wasi --manifest-path rust/Cargo.toml

wasm-run:
	@echo "Usage: make wasm-run CMD=<command> [ARGS=<args>]"
	@echo "Example: make wasm-run CMD='echo hello'"
	@if command -v wasmtime >/dev/null 2>&1; then \
		wasmtime rust/target/wasm32-wasi/release/gvibu.wasm $(CMD) $(ARGS); \
	else \
		echo "wasmtime not found. Install it: curl https://wasmtime.dev/install.sh | bash"; \
		exit 1; \
	fi

wasm-test: wasm-build
	@echo "=== Testing WASM binary via wasmtime ==="
	@if ! command -v wasmtime >/dev/null 2>&1; then \
		echo "wasmtime not found. Install it: curl https://wasmtime.dev/install.sh | bash"; \
		exit 1; \
	fi
	wasmtime rust/target/wasm32-wasi/release/gvibu.wasm true && echo "PASS: true" || echo "FAIL: true"
	wasmtime rust/target/wasm32-wasi/release/gvibu.wasm echo hello | grep -q hello && echo "PASS: echo" || echo "FAIL: echo"
	wasmtime rust/target/wasm32-wasi/release/gvibu.wasm false && echo "FAIL: false" || echo "PASS: false (exit 1)"

wasm-browser-build:
	@if ! command -v wasm-pack >/dev/null 2>&1; then \
		echo "wasm-pack not found. Install it: cargo install wasm-pack"; \
		exit 1; \
	fi
	wasm-pack build wasm-lib --target web --out-dir pkg
	@echo ""
	@echo "WASM browser build complete."
	@echo "Open wasm-lib/demo/index.html in a browser (serve via HTTP, not file://)"

wasm-browser-serve: wasm-browser-build
	@if command -v python3 >/dev/null 2>&1; then \
		echo "Serving demo at http://localhost:8080"; \
		python3 -m http.server 8080 --directory wasm-lib; \
	elif command -v python >/dev/null 2>&1; then \
		echo "Serving demo at http://localhost:8080"; \
		python -m http.server 8080 --directory wasm-lib; \
	else \
		echo "No Python available. Run an HTTP server on wasm-lib/"; \
		exit 1; \
	fi

wasm-browser-clean:
	rm -rf wasm-lib/pkg wasm-lib/target

benchmark: rust-release
	$(PYTHON) tooling/benchmark.py

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

release: rust-release
	@echo "=== gvibu v$(shell grep '^version' rust/Cargo.toml | head -1 | cut -d'"' -f2) ==="
	cp rust/target/release/gvibu gvibu-x86_64-linux
	strip gvibu-x86_64-linux
	tar czf gvibu-x86_64-linux.tar.gz gvibu-x86_64-linux
	@echo "Created gvibu-x86_64-linux.tar.gz"

release-wasm: wasm-browser-build
	cd wasm-lib/pkg && tar czf ../../gvibu-wasm-browser.tar.gz .
	@echo "Created gvibu-wasm-browser.tar.gz"

release-docker:
	docker build -t gvibu:latest -f Dockerfile .
	docker tag gvibu:latest ghcr.io/luluthehungrycat/gvibu-ai-lab:latest
	@echo "To push: docker push ghcr.io/luluthehungrycat/gvibu-ai-lab:latest"

install: rust-release
	cp rust/target/release/gvibu /usr/local/bin/gvibu
		@		for cmd in true false echo pwd basename dirname cat wc head yes printenv sleep touch seq which uname env whoami link unlink tee mkdir rmdir hostname logname readlink realpath uniq uptime id who kill cut tr mv rm ln chmod chown sort grep ls cp printf date expr split tail tac fold comm join nl shuf sum du df test '['; do \
		ln -sf /usr/local/bin/gvibu "/usr/local/bin/$$cmd"; \
	done
	@echo "Installed gvibu and symlinks to /usr/local/bin"

man:
	$(PYTHON) tooling/generate_manpages.py
	@echo "Man pages generated in man/"

status:
	$(PYTHON) tooling/generate_command_status.py

# ── VIBIX kernel binary targets ──────────────────────────────────────────────
# Each kernel/user_<name>.asm assembles to kernel/user_<name>.bin
# Usage: make vibix-<name>   (e.g. vibix-echo, vibix-true)
#        make vibix-all      (build every command)
#        make vibix-clean    (remove all .bin files)

VIBIX_DIR      = kernel
VIBIX_SOURCES  = $(wildcard $(VIBIX_DIR)/user_*.asm)
VIBIX_BINARIES = $(VIBIX_SOURCES:.asm=.bin)
NASM_FLAGS     = -f bin -I $(VIBIX_DIR)/

# Generic pattern: any .asm in kernel/ → .bin
$(VIBIX_DIR)/user_%.bin: $(VIBIX_DIR)/user_%.asm $(wildcard $(VIBIX_DIR)/vibix_*.inc)
	$(NASM) $(NASM_FLAGS) $< -o $@

# Named targets for each command
vibix-echo:    $(VIBIX_DIR)/user_echo_init.bin  # test-harness echo (PID 1)
vibix-echocmd: $(VIBIX_DIR)/user_echo.bin       # thin echo command
vibix-true:    $(VIBIX_DIR)/user_true.bin
vibix-false:   $(VIBIX_DIR)/user_false.bin
vibix-yes:     $(VIBIX_DIR)/user_yes.bin
vibix-clear:   $(VIBIX_DIR)/user_clear.bin
vibix-printenv: $(VIBIX_DIR)/user_printenv.bin
vibix-cat:     $(VIBIX_DIR)/user_cat.bin

vibix-all: $(VIBIX_BINARIES)
	@echo "VIBIX binaries built:"
	ls -1 $(VIBIX_DIR)/user_*.bin

vibix-clean:
	rm -f $(VIBIX_DIR)/user_*.bin

# ── Clean ────────────────────────────────────────────────────────────────────

clean: vibix-clean
	cd rust && $(CARGO) clean
	rm -rf rust/target wasm-lib/pkg wasm-lib/target

help:
	@echo "gvibu build system"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build both implementations"
	@echo "  python-build - Setup Python reference (no build needed)"
	@echo "  rust-build   - Build Rust implementation (debug)"
	@echo "  rust-release - Build Rust implementation (release)"
	@echo "  wasm-build   - Build WASM binary (wasm32-wasi)"
	@echo "  wasm-run     - Run command via wasmtime: make wasm-run CMD='echo hello'"
	@echo "  wasm-test    - Build WASM + run basic tests with wasmtime"
	@echo "  wasm-browser-build - Build browser WASM with wasm-pack (wasm32-unknown-unknown)"
	@echo "  wasm-browser-serve - Build WASM + serve demo page at http://localhost:8080"
	@echo "  wasm-browser-clean - Remove browser WASM build artifacts"
	@echo "  test         - Run comparison tests"
	@echo "  compare      - Compare Python and Rust implementations"
	@echo "  python-test  - Run Python unit tests"
	@echo "  rust-test    - Run Rust unit tests"
	@echo "  benchmark    - Build release + benchmark Python vs Rust"
	@echo "  docker-build - Build Docker container image"
	@echo "  man          - Generate man pages from specs"
	@echo "  initramfs    - Build QEMU initramfs image"
	@echo "  qemu         - Build initramfs and run in QEMU"
	@echo "  install      - Install gvibu binary and symlinks"
	@echo "  release      - Build release binary and package as tar.gz"
	@echo "  release-wasm - Build browser WASM and package as tar.gz"
	@echo "  release-docker - Build and tag Docker image for GHCR"
	@echo "  status       - Show command implementation status"
	@echo "  vibix-all    - Build all VIBIX flat binaries (echo, true, false, yes, clear, printenv, cat)"
	@echo "  vibix-<name> - Build specific VIBIX binary (vibix-echo, vibix-true, vibix-false, etc.)"
	@echo "  vibix-clean  - Remove all VIBIX binary artifacts"
	@echo "  clean        - Clean build artifacts"
