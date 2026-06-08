use std::fs;
use std::io::Write;
use std::path::Path;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    if args.len() != 2 {
        eprintln!("link: exactly two arguments required: FILE LINK");
        return 1;
    }

    let src = &args[0];
    let dst = &args[1];

    if !Path::new(src).exists() {
        eprintln!("link: {}: no such file or directory", src);
        return 1;
    }

    if let Err(e) = fs::hard_link(src, dst) {
        eprintln!("link: cannot create link {} -> {}: {}", dst, src, e);
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_link_one_arg() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 1);
    }

    #[test]
    fn test_link_too_many_args() {
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "b".into(), "c".into()]), 1);
    }

    #[test]
    fn test_link_nonexistent_source() {
        assert_eq!(run(&mut std::io::sink(), &["/nonexistent_link_src".into(), "/tmp/link_dst".into()]), 1);
    }
}
