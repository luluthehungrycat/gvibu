use std::fs;
use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    if args.len() != 1 {
        eprintln!("unlink: exactly one argument required: FILE");
        return 1;
    }

    let path = &args[0];

    if let Err(e) = fs::remove_file(path) {
        eprintln!("unlink: cannot unlink {}: {}", path, e);
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlink_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_unlink_too_many_args() {
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "b".into()]), 1);
    }

    #[test]
    fn test_unlink_nonexistent() {
        assert_eq!(run(&mut std::io::sink(), &["/nonexistent_unlink_test".into()]), 1);
    }
}
