# WASM Support

gvibu commands can run directly in the browser via WebAssembly.

## How It Works

The `wasm-lib/` crate wraps the `gvibu` library with `wasm-bindgen` bindings:

```rust
#[wasm_bindgen]
pub fn run_command(name: &str, args: Vec<String>) -> String {
    let mut buf = Vec::new();
    if let Some(cmd) = commands::lookup(name) {
        (cmd.run)(&mut buf, &args);
    }
    String::from_utf8(buf).unwrap_or_default()
}

#[wasm_bindgen]
pub fn list_commands() -> Vec<String> {
    commands::COMMANDS.iter()
        .flat_map(|c| c.names.iter().map(|n| (*n).to_string()))
        .collect()
}
```

The key enabler is the **writer pattern** — commands write to a `Vec<u8>` buffer instead of stdout.

## Building

```bash
make wasm-browser-build
```

This runs `wasm-pack build --target web` on `wasm-lib/`, producing:
- `wasm-lib/pkg/gvibu_wasm.js` — JavaScript glue
- `wasm-lib/pkg/gvibu_wasm_bg.wasm` — WASM binary

## Running the Demo

```bash
make wasm-browser-serve
# Open http://localhost:8080
```

The demo page (`wasm-lib/index.html`):
- Displays all 51 commands as clickable tiles
- Provides a terminal-style input for command + args
- Shows command output in real time
- Supports Tab-based autocomplete
- Handles error states gracefully

## CI Build

The WASM build is verified in CI via the `wasm` job in `parity.yml`:

```yaml
wasm:
  runs-on: ubuntu-latest
  steps:
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-unknown-unknown
    - run: cargo install wasm-pack
    - run: make wasm-browser-build
    - run: ls wasm-lib/pkg/gvibu_wasm.js wasm-lib/pkg/gvibu_wasm_bg.wasm
```

## npm Package

When a `v*` tag is pushed, the release workflow:

1. Builds the WASM package with `wasm-pack`
2. Patches `package.json` with GitHub Packages scope
3. Publishes to `npm.pkg.github.com/@luluthehungrycat/gvibu-wasm`

## Limitations

- **Filesystem**: WASM runs in a sandbox — commands that read files (`cat`, `head`) cannot access the host filesystem. The demo handles this gracefully.
- **Stdin**: Commands that read stdin (like `tee`, `cat -`) need explicit stdin input. The WASM runner accepts an optional stdin string.
- **System calls**: `who` (utmp), `id` (getuid/getgid), `kill` (kill) — these may not work in all WASM environments, but the bindings compile and return appropriate errors.
