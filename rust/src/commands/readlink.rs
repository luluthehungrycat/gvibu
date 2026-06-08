use std::io::Write;
use std::path::Path;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if args.len() != 1 {
        eprintln!("readlink: missing operand");
        return 1;
    }
    match std::fs::read_link(Path::new(&args[0])) {
        Ok(path) => {
            writeln!(stdout, "{}", path.display()).ok();
            0
        }
        Err(e) => {
            eprintln!("readlink: {}: {}", args[0], e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readlink_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_readlink_too_many_args() {
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "b".into()]), 1);
    }
}
