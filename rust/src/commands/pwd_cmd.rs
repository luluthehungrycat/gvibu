use std::env;

pub fn run(args: &[String]) -> i32 {
    // If any arguments are supplied, treat as usage error (non-optional behavior)
    if !args.is_empty() {
        eprintln!("pwd: usage: pwd");
        return 2;
    }
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            0
        }
        Err(e) => {
            eprintln!("pwd: {}", e);
            1
        }
    }
}
