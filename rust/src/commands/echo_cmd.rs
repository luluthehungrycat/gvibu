pub fn run(args: &[String]) -> i32 {
    let mut newline = true;
    let mut start_idx = 0;

    if let Some(first) = args.first() {
        if first == "-n" {
            newline = false;
            start_idx = 1;
        }
    }

    for (i, arg) in args.iter().enumerate().skip(start_idx) {
        if i > start_idx {
            print!(" ");
        }
        print!("{}", arg);
    }

    if newline {
        println!();
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_no_args() {
        assert_eq!(run(&[]), 0);
    }

    #[test]
    fn test_echo_hello() {
        assert_eq!(run(&["hello".into()]), 0);
    }

    #[test]
    fn test_echo_multiple_args() {
        assert_eq!(run(&["hello".into(), "world".into()]), 0);
    }

    #[test]
    fn test_echo_n_flag() {
        assert_eq!(run(&["-n".into(), "hello".into()]), 0);
    }
}
