use std::ffi::OsString;

pub fn run(args: &[OsString]) -> i32 {
    // No options supported; any arguments are echoed as-is
    let output = args
        .iter()
        .map(|s| s.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
    0
}
