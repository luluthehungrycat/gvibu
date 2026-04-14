.PHONY: all build python-build rust-build test compare clean help

PYTHON := python3
CARGO := $(HOME)/.cargo/bin/cargo

all: python-build rust-build

python-build:
	@echo "Python reference is interpreted - no build needed"

rust-build:
	$(CARGO) build --manifest-path rust/Cargo.toml

test: compare

compare:
	$(PYTHON) tooling/compare_impls.py

python-test:
	@echo "Running Python reference tests..."
	PYTHONPATH=python-ref $(PYTHON) -c "from gvibu_ref.commands import true, false, echo, pwd; print('Python imports OK')"

rust-test:
	$(CARGO) test --manifest-path rust/Cargo.toml

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
	@echo "  rust-build   - Build Rust implementation"
	@echo "  test         - Run comparison tests"
	@echo "  compare      - Compare Python and Rust implementations"
	@echo "  python-test  - Test Python reference"
	@echo "  rust-test    - Test Rust implementation"
	@echo "  status       - Show command implementation status"
	@echo "  clean        - Clean build artifacts"
