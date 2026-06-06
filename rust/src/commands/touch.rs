/// touch: update file timestamps or create empty files.
use std::fs;
use std::io::Write;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("touch: usage: touch FILE...");
        return 2;
    }

    let mut exit_code = 0;

    for fname in args {
        // Open in append mode to create if missing
        match fs::OpenOptions::new().append(true).create(true).open(fname) {
            Ok(mut file) => {
                // Flush to ensure file is written, then set times
                let _ = file.flush();
                if let Err(e) = file.set_modified(std::time::SystemTime::now()) {
                    // Not all platforms support set_modified; ignore if unsupported
                    eprintln!("touch: {}: {}", fname, e);
                    exit_code = 1;
                }
            }
            Err(e) => {
                eprintln!("touch: {}: {}", fname, e);
                exit_code = 1;
            }
        }
    }

    exit_code
}
