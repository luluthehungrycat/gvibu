/// seq: print a sequence of numbers.

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("seq: usage: seq [FIRST [STEP]] LAST");
        return 1;
    }

    let (first, step, last) = match args.len() {
        1 => {
            match args[0].parse::<i64>() {
                Ok(last) => (1i64, 1i64, last),
                Err(_) => {
                    eprintln!("seq: invalid number");
                    return 1;
                }
            }
        }
        2 => {
            match (args[0].parse::<i64>(), args[1].parse::<i64>()) {
                (Ok(first), Ok(last)) => (first, 1i64, last),
                _ => {
                    eprintln!("seq: invalid number");
                    return 1;
                }
            }
        }
        3 => {
            match (args[0].parse::<i64>(), args[1].parse::<i64>(), args[2].parse::<i64>()) {
                (Ok(first), Ok(step), Ok(last)) => (first, step, last),
                _ => {
                    eprintln!("seq: invalid number");
                    return 1;
                }
            }
        }
        _ => {
            eprintln!("seq: too many arguments");
            return 1;
        }
    };

    if step == 0 {
        eprintln!("seq: step cannot be zero");
        return 1;
    }

    if step > 0 && first > last {
        return 0;
    }
    if step < 0 && first < last {
        return 0;
    }

    let mut i = first;
    if step > 0 {
        while i <= last {
            println!("{}", i);
            i += step;
        }
    } else {
        while i >= last {
            println!("{}", i);
            i += step;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_no_args() {
        assert_eq!(run(&[]), 1);
    }

    #[test]
    fn test_seq_single() {
        assert_eq!(run(&["5".into()]), 0);
    }

    #[test]
    fn test_seq_first_last() {
        assert_eq!(run(&["3".into(), "7".into()]), 0);
    }

    #[test]
    fn test_seq_full() {
        assert_eq!(run(&["2".into(), "3".into(), "14".into()]), 0);
    }

    #[test]
    fn test_seq_first_greater() {
        assert_eq!(run(&["10".into(), "5".into()]), 0);
    }

    #[test]
    fn test_seq_negative_step() {
        assert_eq!(run(&["10".into(), "-2".into(), "4".into()]), 0);
    }

    #[test]
    fn test_seq_step_zero() {
        assert_eq!(run(&["1".into(), "0".into(), "5".into()]), 1);
    }

    #[test]
    fn test_seq_invalid() {
        assert_eq!(run(&["abc".into()]), 1);
    }
}
