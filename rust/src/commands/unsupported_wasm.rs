use std::io::Write;

/// Command implementations that depend on host operating-system facilities
/// are replaced with this explicit error on WebAssembly targets.
pub fn run(stdout: &mut dyn Write, _args: &[String]) -> i32 {
    let command = module_path!().rsplit("::").next().unwrap_or("command");
    // The browser binding captures stdout but has no separate stderr channel.
    // Include the reason in captured output so callers see why it failed.
    let _ = writeln!(stdout, "{}: not supported on WebAssembly", command);
    1
}
