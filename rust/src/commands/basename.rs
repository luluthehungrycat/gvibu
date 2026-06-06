use std::string::ToString;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() || (args.len() == 1 && args[0].is_empty()) {
        eprintln!("basename: usage: basename NAME [SUFFIX]");
        return 1;
    }
    let mut name = args[0].trim_end_matches('/').to_string();
    // Extract final path component (basename) first
    if let Some(pos) = name.rfind('/') {
        name = name[(pos + 1)..].to_string();
    }
    // If there are suffix args, remove if present
    if args.len() > 1 {
        let suffix = &args[1];
        if !suffix.is_empty() && name.ends_with(suffix) {
            name.truncate(name.len() - suffix.len());
        }
    }
    println!("{}", name);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basename_no_args() {
        assert_eq!(run(&[]), 1);
    }

    #[test]
    fn test_basename_simple() {
        assert_eq!(run(&["/usr/local/bin/test.txt".into()]), 0);
    }

    #[test]
    fn test_basename_with_suffix() {
        assert_eq!(run(&["/usr/local/bin/test.txt".into(), ".txt".into()]), 0);
    }

    #[test]
    fn test_basename_root() {
        assert_eq!(run(&["/".into()]), 0);
    }

    #[test]
    fn test_basename_trailing_slash() {
        assert_eq!(run(&["/a/b/c/".into()]), 0);
    }
}
