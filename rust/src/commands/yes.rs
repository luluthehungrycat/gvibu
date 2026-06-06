/// yes: output a string repeatedly.
use std::io::{self, Write};

pub fn run(args: &[String]) -> i32 {
    let text = if args.is_empty() {
        "y".to_string()
    } else {
        args.join(" ")
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        if writeln!(handle, "{}", text).is_err() {
            // Broken pipe — ignore silently
            break;
        }
    }

    0
}
