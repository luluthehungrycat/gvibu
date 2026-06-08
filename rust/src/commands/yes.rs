/// yes: output a string repeatedly.
use std::io::{self, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let text = if args.is_empty() {
        "y".to_string()
    } else {
        args.join(" ")
    };

    loop {
        pwriteln!(w, "{}", text);
    }

    0
}
