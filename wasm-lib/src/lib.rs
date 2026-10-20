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
///   stdin  - Optional stdin input; missing input is treated as empty stdin.
///
/// Returns the command's stdout output as a string.
///
/// Stdin-reading commands consume the supplied text, treating a missing value
/// as empty input. Browser WASM does not expose a host filesystem, so file
/// operations may fail. Host-specific commands (including `ls`, `chmod`,
/// `id`, `who`, `kill`, `df`, `uname`, `uptime`, `ln`, `which`, and `split`)
/// return an explicit unsupported-platform error.
#[wasm_bindgen]
pub fn run_command(name: &str, args: Vec<String>, stdin: Option<String>) -> Result<String, JsValue> {
    dispatch_command(name, &args, stdin.as_deref()).map_err(|error| JsValue::from_str(&error))
}

fn dispatch_command(name: &str, args: &[String], stdin: Option<&str>) -> Result<String, String> {
    // A browser call always owns its stdin. Missing input means an empty
    // buffer, never the unavailable host process stdin.
    gvibu::set_wasm_stdin(Some(stdin.unwrap_or("")));
    let result = (|| {
        let cmd = gvibu::commands::lookup(name)
            .ok_or_else(|| format!("unknown command: {}", name))?;

        let mut writer = WasmWriter::new();
        let exit_code = (cmd.run)(&mut writer, &args);
        let output = writer.into_string();

        if exit_code == 0 {
            Ok(output)
        } else {
            Err(format!(
                "Command '{}' exited with code {}:\n{}",
                name, exit_code, output
            ))
        }
    })();
    gvibu::clear_wasm_stdin();
    result
}

#[cfg(test)]
mod tests {
    use super::dispatch_command;

    #[test]
    fn each_call_gets_its_own_stdin_and_clears_it_afterward() {
        assert_eq!(dispatch_command("wc", &["-l".into()], Some("one\n")), Ok("      1\n".into()));
        assert_eq!(dispatch_command("wc", &["-l".into()], Some("one\ntwo\n")), Ok("      2\n".into()));
        assert_eq!(dispatch_command("wc", &["-l".into()], None), Ok("      0\n".into()));
        assert!(dispatch_command("missing", &[], Some("unused")).is_err());
        assert_eq!(gvibu::get_wasm_stdin(), None);
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
