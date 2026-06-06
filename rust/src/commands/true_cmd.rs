pub fn run(args: &[String]) -> i32 {
    let _ = args;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_no_args() {
        assert_eq!(run(&[]), 0);
    }

    #[test]
    fn test_true_with_args() {
        assert_eq!(run(&["a".into(), "b".into()]), 0);
    }
}
