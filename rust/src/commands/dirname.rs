/// dirname: strip last component from a file path.
pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("dirname: usage: dirname NAME");
        return 2;
    }

    let path = &args[0];

    // Empty string -> current directory
    if path.is_empty() {
        println!(".");
        return 0;
    }

    // Strip trailing slashes
    let stripped = path.trim_end_matches('/');

    // If entirely slashes -> root
    if stripped.is_empty() {
        println!("/");
        return 0;
    }

    // Find last '/'
    match stripped.rfind('/') {
        None => {
            // No slash: return current directory
            println!(".");
        }
        Some(pos) => {
            if pos == 0 {
                // Slash at start: root
                println!("/");
            } else {
                // Everything before the last slash
                println!("{}", &stripped[..pos]);
            }
        }
    }

    0
}
