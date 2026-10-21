# gvibu — Universal Coreutils in Rust

**gvibu** is a reimplementation of standard Unix coreutils in Rust, featuring a Python reference implementation for parity testing, WASM browser support, and full CI/CD.

## Features

- **60 commands** — from `true`/`false` to `cp`/`printf`/`date`/`expr`/`split`, `ls`/`grep`/`du`/`df`, `sort`/`test`/`[`, `expand`/`rev`, tail, fold, join, and more, all with core flag support
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

### Text & Sorting (12)
`grep` `fold` `expand` `rev` `comm` `join` `nl` `shuf` `sort` `printf` `expr` `date`

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
│   ├── src/commands/     # 60 command modules
│   ├── src/main.rs       # Multicall binary dispatch
│   ├── src/lib.rs        # Library entry (for WASM)
│   └── tests/
│       ├── cli.rs        # 332+ integration tests
│       └── fuzz.rs       # 24 property-based tests
├── python-ref/           # Python reference implementation
├── specs/commands/       # 60 command specifications
├── shared-tests/cases/   # Shared test cases (both impls)
├── tests/                # Python test suite
├── wasm-lib/             # WASM bindings + browser demo
│   └── index.html        # Interactive terminal
├── tooling/              # Utility scripts
├── gvibu-linux/          # QEMU/initramfs environment
├── vibix-lib/             # no-std VIBIX runtime and flat command binary
├── docs/                  # Architecture & design docs
├── man/                   # Generated man pages
├── ROADMAP.md             # maintained stack and image integration contract
└── CHANGELOG.md           # dated project milestones and verification
```

## Architecture

All commands are compiled into a single **multicall binary**. Dispatch works via two modes:

1. **Symlink mode** — argv[0] basename determines the command
2. **Subcommand mode** — `gvibu <command> [args...]`

Every Rust command exports `pub fn run(w: &mut dyn Write, args: &[String]) -> i32`, where `w` is either `stdout` (CLI) or a captured buffer (WASM).

### GVIBU/VIBIX runtime boundary

GVIBU owns command behavior, flags, stdout/stderr bytes, and exit statuses.
The no-std `vibix-lib/` crate owns the shared VIBIX user runtime: raw syscall
bindings, typed sentinel/errno conversion, non-negative descriptors, checked
I/O, and the `brk` boundary. VIBIT owns init/reaping, VISH owns shell policy,
and VIBIX owns the kernel ABI.

The first checked consumer is the VIBIX `echo` path. Its successful flags and
output remain defined by `specs/commands/echo.md`; a failed output write maps
to the existing runtime-error status `1`. Linux and WASM continue to use the
writer-based `rust/` and `wasm-lib/` surfaces.

Compilation is not a VIBIX portability claim. A portability claim requires
the runtime checks, command/parity tests, and the cross-repository
VIBIT → VISH → GVIBU QEMU path, including output status, redirection or pipes,
child reaping, and clean exit.

## Performance

| Command | Rust  | Python | Speedup |
|---------|-------|--------|---------|
| true    | 0.1ms | 11ms   | 110×    |
| echo    | 0.2ms | 12ms   | 60×     |
| seq     | 0.3ms | 34ms   | 113×    |
| uniq    | 0.2ms | 10ms   | 50×     |
| ...     |       |        | 28–113× |

Run `make benchmark` for full results.

## License

MIT
