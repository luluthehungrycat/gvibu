use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = (stdout, args);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_false_with_args() {
        assert_eq!(run(&mut std::io::sink(), &["a".into()]), 1);
    }
}
