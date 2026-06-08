# Architecture

## Multicall Binary Pattern

gvibu uses the **multicall binary** pattern: a single compiled binary serves as all 58 commands. This is the same pattern used by BusyBox and toybox.

### Dispatch Flow

```
argv[0] = "echo"
         │
         ▼
 resolve_command()
         │
         ├── Symlink mode: look up argv[0] basename in COMMANDS table
         │   e.g., symlink "echo" → "echo" → echo::run()
         │
         └── Subcommand mode: argv[0] is binary name, argv[1] is command
             e.g., "gvibu echo hello" → "echo" → echo::run(["hello"])
```

### Command Registration

All commands are registered in `rust/src/commands/mod.rs`:

```rust
pub const COMMANDS: &[Command] = &[
    Command { names: &["true"], run: true_cmd::run },
    Command { names: &["false"], run: false_cmd::run },
    // ... 49 entries
];
```

The `Command` struct:
```rust
pub struct Command {
    pub names: &'static [&'static str],  // Aliases (e.g., test and [)
    pub run: fn(&mut dyn Write, &[String]) -> i32,
}
```

## Writer-Pattern Architecture

All commands accept a `&mut dyn Write` parameter instead of writing directly to stdout:

```rust
pub fn run(w: &mut dyn Write, args: &[String]) -> i32
```

This enables the **same code** to work in two contexts:
1. **CLI mode** — `main.rs` passes `&mut io::stdout()`
2. **WASM mode** — the WASM wrapper passes a `Vec<u8>` buffer

### Why This Matters

Without the writer parameter, WASM support would require either:
- Duplicating all command logic
- Replacing stdout at the OS level (not possible in WASM)

With the writer parameter, WASM support is a 1-line wrapper:

```rust
#[wasm_bindgen]
pub fn run_command(name: &str, args: Vec<String>) -> String {
    let mut buf = Vec::new();
    if let Some(cmd) = commands::lookup(name) {
        (cmd.run)(&mut buf, &args);
    }
    String::from_utf8(buf).unwrap_or_default()
}
```

## Library Structure

The Rust crate has both a library and a binary target:

```
Cargo.toml
  ├── [lib] → src/lib.rs → pub mod commands
  └── [[bin]] → src/main.rs → use gvibu::commands;
```

The library target (`lib.rs`) is minimal:
```rust
pub mod commands;
```

This allows `wasm-lib/` to depend on `gvibu` as a library, while the CLI binary is a thin shell that calls into the same code.

## File Layout

```
rust/
├── src/
│   ├── lib.rs              # Library root (re-exports commands)
│   ├── main.rs             # Binary entry (dispatch + CLI)
│   └── commands/
│       ├── mod.rs          # Command table + lookup
│       ├── echo_cmd.rs     # Individual command modules
│       ├── cat.rs
│       └── ... (49 files)
├── tests/
│   ├── cli.rs              # Integration tests (subprocess)
│   └── fuzz.rs             # Property-based tests (proptest)
└── Cargo.toml
```
