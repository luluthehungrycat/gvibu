/// dirname: strip last component from a file path.
use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("dirname: usage: dirname NAME");
        return 2;
    }

    let path = &args[0];

    // Empty string -> current directory
    if path.is_empty() {
        writeln!(stdout, ".").ok();
        return 0;
    }

    // Strip trailing slashes
    let stripped = path.trim_end_matches('/');

    // If entirely slashes -> root
    if stripped.is_empty() {
        writeln!(stdout, "/").ok();
        return 0;
    }

    // Find last '/'
    match stripped.rfind('/') {
        None => {
            // No slash: return current directory
            writeln!(stdout, ".").ok();
        }
        Some(pos) => {
            if pos == 0 {
                // Slash at start: root
                writeln!(stdout, "/").ok();
            } else {
                // Everything before the last slash
                writeln!(stdout, "{}", &stripped[..pos]).ok();
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirname_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 2);
    }

    #[test]
    fn test_dirname_normal() {
        assert_eq!(run(&mut std::io::sink(), &["/usr/local/bin".into()]), 0);
    }

    #[test]
    fn test_dirname_root_parent() {
        assert_eq!(run(&mut std::io::sink(), &["/usr".into()]), 0);
    }

    #[test]
    fn test_dirname_root() {
        assert_eq!(run(&mut std::io::sink(), &["/".into()]), 0);
    }

    #[test]
    fn test_dirname_simple() {
        assert_eq!(run(&mut std::io::sink(), &["foo".into()]), 0);
    }

    #[test]
    fn test_dirname_relative() {
        assert_eq!(run(&mut std::io::sink(), &["a/b".into()]), 0);
    }
}
