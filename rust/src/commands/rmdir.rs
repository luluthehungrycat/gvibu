use std::fs;
use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    if args.is_empty() {
        eprintln!("rmdir: missing operand");
        return 1;
    }

    let mut exit_code = 0;
    for dir in args {
        if let Err(e) = fs::remove_dir(dir) {
            eprintln!("rmdir: failed to remove '{}': {}", dir, e);
            exit_code = 1;
        }
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmdir_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_rmdir_nonexistent() {
        assert_eq!(run(&mut std::io::sink(), &["/tmp/nonexistent_rmdir_test_xyz".into()]), 1);
    }
}
