use std::ffi::OsString;
use std::path::PathBuf;

pub fn run(args: &[OsString]) -> i32 {
    // No options supported; extra arguments are a usage error
    if !args.is_empty() {
        eprintln!("pwd: usage: pwd");
        return 2;
    }

    match std::env::current_dir() {
        Ok(dir) => {
            println!("{}", dir.display());
            0
        }
        Err(e) => {
            eprintln!("pwd: {}", e);
            1
        }
    }
}
