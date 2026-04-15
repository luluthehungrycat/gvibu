fn route(cmd: &str, sub: &str) -> String {
    match (cmd, sub) {
        ("gvibu", "help") => {
            "GVIBU Rust surface: available subcommands: help, init, status, run".to_string()
        }
        ("gvibu", "init") => "GVIBU Rust init: placeholder".to_string(),
        ("gvibu", "status") => "GVIBU Rust status: placeholder".to_string(),
        ("gvibu", "run") => "GVIBU Rust run: placeholder".to_string(),
        _ => format!("Unknown GVIBU Rust subcommand: {} {}", cmd, sub),
    }
}

mod gvibu;

use std::env;

fn main() {
    // Minimal Rust GVIBU surface: supports a gvibu subcommand mirroring the Python surface
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "gvibu" {
        let sub = if args.len() >= 3 { &args[2] } else { "help" };
        let out = gvibu::route("gvibu", sub);
        println!("{}", out);
        return;
    }
    println!("GVIBU Rust vish: placeholder");
}
