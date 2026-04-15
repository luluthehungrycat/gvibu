pub fn route(cmd: &str, sub: &str) -> String {
    // Read version from environment at runtime (set by build script if available)
    let ver = std::env::var("GVIBU_RUST_VERSION").unwrap_or_else(|_| "dev".to_string());
    match (cmd, sub) {
        ("gvibu", "help") => {
            "GVIBU Rust surface: available subcommands: help, init, status, run".to_string()
        }
        ("gvibu", "init") => "GVIBU Rust init: placeholder".to_string(),
        ("gvibu", "status") => "GVIBU Rust status: placeholder".to_string(),
        ("gvibu", "run") => "GVIBU Rust run: placeholder".to_string(),
        ("gvibu", "version") => format!("GVIBU Rust surface version {}", ver),
        _ => format!("Unknown GVIBU Rust subcommand: {} {}", cmd, sub),
    }
}
