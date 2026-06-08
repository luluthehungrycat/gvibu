use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if args.len() != 1 {
        eprintln!("realpath: missing operand");
        return 1;
    }
    match std::fs::canonicalize(&args[0]) {
        Ok(path) => {
            writeln!(stdout, "{}", path.display()).ok();
            0
        }
        Err(e) => {
            eprintln!("realpath: {}: {}", args[0], e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realpath_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_realpath_nonexistent() {
        assert_eq!(run(&mut std::io::sink(), &["/nonexistent_path_xyz".into()]), 1);
    }
}
