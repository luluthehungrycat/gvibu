use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = (stdout, args);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }

    #[test]
    fn test_true_with_args() {
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "b".into()]), 0);
    }
}
