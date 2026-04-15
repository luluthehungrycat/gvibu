use std::process::Command;

#[test]
fn test_rust_gvibu_help() {
    // Build path and attempt to run the gvibu help path
    let bin_path = "./target/debug/gvibu_rust";
    if std::path::Path::new(bin_path).exists() {
        let output = Command::new(bin_path)
            .arg("gvibu")
            .arg("help")
            .output()
            .expect("failed to execute rust gvibu binary");
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("GVIBU Rust"));
    } else {
        // If the binary isn't built in CI, skip the runtime test gracefully
        println!("SKIP: Rust binary not built in this environment");
    }
}
