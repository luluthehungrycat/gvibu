pub fn run(args: &[String]) -> i32 {
    let _ = args;
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false_no_args() {
        assert_eq!(run(&[]), 1);
    }

    #[test]
    fn test_false_with_args() {
        assert_eq!(run(&["a".into()]), 1);
    }
}
