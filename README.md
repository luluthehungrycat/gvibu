# gvibu — Universal Coreutils in Rust

**gvibu** is a reimplementation of standard Unix coreutils in Rust, featuring a Python reference implementation for parity testing, WASM browser support, and full CI/CD.

## Features

- **58 commands** — from `true`/`false` to `cp`/`printf`/`date`/`expr`/`split`, `ls`/`grep`/`du`/`df`, `sort`/`test`/`[`, tail, fold, join, and more, all with core flag support
- **Fast** — Rust implementation is 28–113× faster than the Python reference
- **WASM** — run commands directly in your browser via `wasm-bindgen`
- **Dual implementation** — Rust (production) + Python (reference) with shared parity tests
- **Fuzz-tested** — 24 property-based tests via `proptest`
- **Portable** — Docker, QEMU/initramfs bootable image, static binary builds
- **CI/CD** — GitHub Actions with test, WASM, Docker, and release workflows

## Commands

### Core (17)
`true` `false` `echo` `pwd` `basename` `dirname` `cat` `wc` `head` `yes` `printenv` `sleep` `touch` `seq` `which` `uname` `env`

### File Operations (14)
`whoami` `link` `unlink` `tee` `mkdir` `rmdir` `tail` `tac` `sum` `du` `df` `ls` `cp` `split`

### System Info (6)
`hostname` `logname` `readlink` `realpath` `uniq` `uptime`

### User & Process (8)
`id` `who` `kill` `cut` `tr` `mv` `rm` `ln`

### Text & Sorting (10)
`grep` `fold` `comm` `join` `nl` `shuf` `sort` `printf` `expr` `date`

### Permissions & Conditions (4)
`chmod` `chown` `test` (also `[`)

## Quick Start

```bash
# Build
cargo build --release

# Subcommand mode
./target/release/gvibu echo hello world

# Symlink mode (all 58 commands)
make install
gvibu-echo hello world

# Or just test it
make rust-test
```

## WASM Browser Demo

```bash
make wasm-browser-build
make wasm-browser-serve
# → http://localhost:8080
```

## Testing

```bash
make rust-test       # Rust unit + integration tests
make python-test     # Python reference tests
make compare         # Parity tests (both implementations)
make fuzz            # Property-based fuzzing
make benchmark       # Rust vs Python speed comparison
```

## Project Structure

```
gvibu-ai-lab/
├── rust/                 # Rust implementation
│   ├── src/commands/     # 58 command modules
│   ├── src/main.rs       # Multicall binary dispatch
│   ├── src/lib.rs        # Library entry (for WASM)
│   └── tests/
│       ├── cli.rs        # 332+ integration tests
│       └── fuzz.rs       # 24 property-based tests
├── python-ref/           # Python reference implementation
├── specs/commands/       # 58 command specifications
├── shared-tests/cases/   # Shared test cases (both impls)
├── tests/                # Python test suite
├── wasm-lib/             # WASM bindings + browser demo
│   └── index.html        # Interactive terminal
├── tooling/              # Utility scripts
├── gvibu-linux/          # QEMU/initramfs environment
├── docs/                 # Architecture & design docs
└── man/                  # Generated man pages
```

## Architecture

All commands are compiled into a single **multicall binary**. Dispatch works via two modes:

1. **Symlink mode** — argv[0] basename determines the command
2. **Subcommand mode** — `gvibu <command> [args...]`

Every Rust command exports `pub fn run(w: &mut dyn Write, args: &[String]) -> i32`, where `w` is either `stdout` (CLI) or a captured buffer (WASM).

## Performance

| Command | Rust  | Python | Speedup |
|---------|-------|--------|---------|
| true    | 0.1ms | 11ms   | 110×    |
| echo    | 0.2ms | 12ms   | 60×     |
| seq     | 0.3ms | 34ms   | 113×    |
| uniq    | 0.2ms | 10ms   | 50×     |
| ...     |       |        | 28–113× |

Run `make benchmark` for full results.

## Linux Userspace Testing (QEMU + initramfs)

In addition to the standard Rust + Python test suites, gvibu can be validated
end-to-end as a Linux userspace by booting a QEMU VM with a custom initramfs
that contains the gvibu toolchain and vish shell.

The boot path is fully scripted and reproducible. There are two variants —
Python and Rust — each producing an initramfs that boots the corresponding
`vish` REPL inside a QEMU x86_64 VM:

```bash
# Prerequisites: qemu-system-x86_64 on PATH, a Linux kernel image, build tools
#   Debian/Ubuntu : sudo apt-get install qemu-system-x86
#   Fedora/RHEL   : sudo dnf install qemu-system-x86

# Build the Rust vish and boot it in QEMU (auto-builds initramfs at /tmp/gvibu_rs_initramfs.cpio)
./gvibu-linux/qemu_run_rs.sh /path/to/vmlinuz

# Build the Python vish and boot it in QEMU (auto-builds /tmp/gvibu_py_initramfs.cpio)
./gvibu-linux/qemu_run_py.sh /path/to/vmlinuz

# Or just print the QEMU command line / run a base initramfs without rebuilding
./gvibu-linux/run_qemu.sh /path/to/vmlinuz
```

Each script:

1. Validates the kernel image path and exits with a clear error if missing.
2. Verifies `qemu-system-x86_64` is on PATH and prints distro-aware install
   instructions if it is not.
3. (Re)builds the variant-specific initramfs if the file is missing or any
   source file (`vish/src/main.rs`, `vish/Cargo.toml`, or
   `gvibu-python/vish.py`) is newer than the existing cpio.
4. Walks the binary's `ldd` output and copies the matching dynamic libraries
   into the initramfs root so the binary can run without a host filesystem.
5. Repacks the cpio with `cpio -o -H newc` and hands off to
   `gvibu-linux/run_qemu.sh` for the actual QEMU launch.

The boot smoke test inside the VM is the `init` script produced by
`gvibu-linux/build_initramfs.sh`, which exercises `gvibu true`, `gvibu false`,
symlink dispatch, and a curated set of representative commands.

## License

MIT
