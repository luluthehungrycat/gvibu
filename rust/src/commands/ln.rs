/// ln: make links between files.
use std::io::Write;
use std::os::unix::fs::symlink;
use std::fs;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut symbolic = false;
    let mut force = false;
    let mut verbose = false;
    let mut targets: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg == "-s" {
            symbolic = true;
        } else if arg == "-f" {
            force = true;
        } else if arg == "-v" {
            verbose = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("ln: invalid option: {}", arg);
            return 1;
        } else {
            targets.push(arg.clone());
        }
        i += 1;
    }

    if targets.len() < 2 {
        eprintln!("ln: missing operand");
        return 1;
    }

    let src = &targets[0];
    let dst = &targets[1];

    if force && (fs::metadata(dst).is_ok() || fs::symlink_metadata(dst).is_ok()) {
        let _ = fs::remove_file(dst);
    }

    let result = if symbolic {
        symlink(src, dst)
    } else {
        fs::hard_link(src, dst)
    };

    match result {
        Ok(_) => {
            if verbose {
                let _ = writeln!(stdout, "linked '{}' -> '{}'", dst, src);
            }
            0
        }
        Err(e) => {
            eprintln!("ln: failed to create link '{}' -> '{}': {}", dst, src, e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ln_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_ln_one_arg() {
        assert_eq!(run(&mut std::io::sink(), &["a".into()]), 1);
    }

    #[test]
    fn test_ln_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }
}
