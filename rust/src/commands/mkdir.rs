use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    let mut parent = false;
    let mut dirs: Vec<String> = Vec::new();

    for arg in args {
        if arg == "--" { break; }
        if arg == "-p" {
            parent = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("mkdir: invalid option: {}", arg);
            return 1;
        } else {
            dirs.push(arg.clone());
        }
    }

    if dirs.is_empty() {
        eprintln!("mkdir: missing operand");
        return 1;
    }

    let mut exit_code = 0;
    for dir in &dirs {
        if parent {
            if let Err(e) = fs::create_dir_all(dir) {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
                exit_code = 1;
            }
        } else {
            if Path::new(dir).exists() {
                eprintln!("mkdir: cannot create directory '{}': File exists", dir);
                exit_code = 1;
            } else if let Err(e) = fs::create_dir(dir) {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
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
    fn test_mkdir_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_mkdir_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into(), "dir".into()]), 1);
    }

    #[test]
    fn test_mkdir_p_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-p".into(), "/tmp/gvibu_test_mkdir_p".into()]), 0);
    }
}
