/// which: locate a command by searching PATH.
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        return 1;
    }

    let path = env::var("PATH").unwrap_or_default();
    let directories: Vec<&str> = path.split(':').filter(|d| !d.is_empty()).collect();

    let mut found = false;

    for name in args {
        for directory in &directories {
            let full_path = format!("{}/{}", directory, name);
            if let Ok(metadata) = fs::metadata(&full_path) {
                if metadata.is_file() {
                    let perms = metadata.permissions();
                    // Check if executable by owner
                    if perms.mode() & 0o111 != 0 {
                        println!("{}", full_path);
                        found = true;
                        break;
                    }
                }
            }
        }
    }

    if found { 0 } else { 1 }
}
