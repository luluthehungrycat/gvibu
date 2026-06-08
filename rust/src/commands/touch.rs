/// touch: update file timestamps or create empty files.
use std::fs;
use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    if args.is_empty() {
        eprintln!("touch: usage: touch FILE...");
        return 2;
    }

    let mut flag_a = false;
    let mut flag_m = false;
    let mut files: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'a' => flag_a = true,
                    'm' => flag_m = true,
                    _ => {
                        eprintln!("touch: invalid option: -{}", ch);
                        return 1;
                    }
                }
            }
        } else {
            files.push(arg.as_str());
        }
        i += 1;
    }

    if files.is_empty() {
        eprintln!("touch: usage: touch FILE...");
        return 2;
    }

    // Default: update both if neither specified
    if !flag_a && !flag_m {
        flag_a = true;
        flag_m = true;
    }

    let now = std::time::SystemTime::now();
    let mut exit_code = 0;

    for fname in files {
        // Open to create if missing (append mode)
        match fs::OpenOptions::new().append(true).create(true).open(fname) {
            Ok(mut file) => {
                let _ = file.flush();
                if flag_a {
                    if let Err(e) = file.set_accessed(now) {
                        eprintln!("touch: {}: setting access time: {}", fname, e);
                        exit_code = 1;
                    }
                }
                if flag_m {
                    if let Err(e) = file.set_modified(now) {
                        eprintln!("touch: {}: setting modification time: {}", fname, e);
                        exit_code = 1;
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 2);
    }

    #[test]
    fn test_touch_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into(), "/tmp/test".into()]), 1);
    }

    #[test]
    fn test_touch_flag_a() {
        assert_eq!(run(&mut std::io::sink(), &["-a".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_touch_flag_m() {
        assert_eq!(run(&mut std::io::sink(), &["-m".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_touch_flag_am() {
        assert_eq!(run(&mut std::io::sink(), &["-am".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_touch_no_flags() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }
}
