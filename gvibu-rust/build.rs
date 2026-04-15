use std::fs;

fn main() {
    // Read version from Cargo.toml and propagate to rustc env var
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap_or_default();
    for line in cargo_toml.lines() {
        if line.trim().starts_with("version") {
            // line like: version = "0.1.0"
            if let Some(v) = line.split('=').nth(1) {
                let v = v.trim().trim_matches('"');
                println!("cargo:rustc-env=GVIBU_RUST_VERSION={}", v);
                break;
            }
        }
    }
}
