#[test]
fn test_lib_gvibu_help() {
    extern crate gvibu_rust_lib;
    use gvibu_rust_lib::route;
    let s = route("gvibu", "help");
    assert!(s.contains("GVIBU Rust"));
}
