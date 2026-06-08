use std::io::Write;
use wasm_bindgen::prelude::*;

/// WasmWriter captures all output written to it into an internal Vec<u8> buffer.
struct WasmWriter {
    buf: Vec<u8>,
}

impl WasmWriter {
    fn new() -> Self {
        WasmWriter { buf: Vec::new() }
    }

    fn into_string(self) -> String {
        String::from_utf8_lossy(&self.buf).to_string()
    }
}

impl Write for WasmWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Run a single gvibu command in the browser.
///
/// Arguments:
///   name   - Command name (e.g. "echo", "true", "pwd")
///   args   - Array of command arguments (excluding the command name itself)
///   _stdin - Optional stdin input string (unused currently; commands that
///            read stdin will panic in WASM)
///
/// Returns the command's stdout output as a string.
///
/// Commands requiring filesystem access (touch, cat FILE, wc FILE) or
/// interactive stdin will panic in the browser WASM environment.
/// Non-filesystem commands (echo, true, false, whoami, pwd, basename,
/// dirname, printenv, seq, yes) work fully.
#[wasm_bindgen]
pub fn run_command(name: &str, args: Vec<String>, _stdin: Option<String>) -> Result<String, JsValue> {
    // Lookup and run the command
    let cmd = gvibu::commands::lookup(name)
        .ok_or_else(|| JsValue::from_str(&format!("unknown command: {}", name)))?;

    let mut writer = WasmWriter::new();
    let exit_code = (cmd.run)(&mut writer, &args);
    let output = writer.into_string();

    if exit_code == 0 {
        Ok(output)
    } else {
        Err(JsValue::from_str(&format!(
            "Command '{}' exited with code {}:\n{}",
            name, exit_code, output
        )))
    }
}

/// List all available command names (primary names).
#[wasm_bindgen]
pub fn list_commands() -> Vec<String> {
    gvibu::commands::COMMANDS
        .iter()
        .filter_map(|cmd| cmd.names.first().map(|s| s.to_string()))
        .collect()
}
