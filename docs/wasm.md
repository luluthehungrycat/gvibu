# WASM Support

gvibu supports two WebAssembly targets: a WASI CLI (`wasm32-wasip1`) and browser bindings (`wasm32-unknown-unknown`). The WASI CLI can use process stdin and files exposed by the host. The browser binding receives stdin as an argument and has no host filesystem by default.

## Browser binding

The `wasm-lib/` crate exposes `run_command(name, args, stdin)` and `list_commands()` through `wasm-bindgen`. Each call supplies its own stdin text; a missing value means empty stdin. The binding captures stdout and returns command failures as JavaScript errors.

Build it with:

```sh
make wasm-browser-build
```

This creates `wasm-lib/pkg/gvibu_wasm.js` and `wasm-lib/pkg/gvibu_wasm_bg.wasm`. Start the demo with:

```sh
make wasm-browser-serve
```

The demo lists all 60 registered commands, accepts command arguments and optional stdin text, and displays output or errors.

## WASI CLI

Build the CLI with:

```sh
cargo build --locked --manifest-path rust/Cargo.toml --target wasm32-wasip1 --release
```

Run it with Wasmtime:

```sh
wasmtime run --argv0 gvibu rust/target/wasm32-wasip1/release/gvibu.wasm true
wasmtime run --argv0 gvibu rust/target/wasm32-wasip1/release/gvibu.wasm echo hello
printf 'alpha\nbeta\n' | wasmtime run --argv0 gvibu rust/target/wasm32-wasip1/release/gvibu.wasm cat
```

WASI filesystem commands can access directories preopened by the runtime. For example, with a recent Wasmtime release:

```sh
wasmtime run --dir ./fixtures::/fixtures --argv0 gvibu rust/target/wasm32-wasip1/release/gvibu.wasm cat /fixtures/input.txt
```

## Platform limits

The browser target cannot access host process stdin or a host filesystem. Use the binding’s stdin argument for input-only commands. Commands that read or write paths depend on host support and may fail when no virtual filesystem is available. `split` is explicitly unsupported in browser WASM because it consumes process stdin and creates files.

Commands that depend on Unix identity, process, permission, mount, or system APIs report that they are unsupported on WebAssembly. This includes `id`, `who`, `kill`, `ls`, `chmod`, `chown`, `ln`, `which`, `uname`, `uptime`, and `df`. Native builds retain these commands.

The `wasm` CI job builds the WASI CLI and browser package, checks WASI stdin and preopened-file behavior, and runs a Node smoke test against the browser binding.
