/// seq: print a sequence of numbers.

fn parse_args(args: &[String]) -> Result<(i64, i64, i64, String, bool), i32> {
    let mut separator = "\n".to_string();
    let mut equal_width = false;
    let mut pos_args: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" {
            break;
        }
        if arg == "-w" {
            equal_width = true;
            i += 1;
        } else if arg == "-s" {
            i += 1;
            if i >= args.len() {
                eprintln!("seq: option requires an argument: -s");
                return Err(1);
            }
            separator = args[i].clone();
            i += 1;
        } else if arg.starts_with('-') && arg.len() > 1 {
            // Check if it is a negative number (all remaining chars are digits)
            if arg[1..].chars().all(|c| c.is_ascii_digit()) {
                pos_args.push(arg);
                i += 1;
                continue;
            }
            // Combined flags like -ws
            for (j, ch) in arg[1..].chars().enumerate() {
                match ch {
                    'w' => equal_width = true,
                    's' => {
                        // -s inside combined flags: rest of this arg is the separator
                        // e.g., -s: or -s,
                        let rest: String = arg[1..].chars().skip(j + 1).collect();
                        if rest.is_empty() {
                            i += 1;
                            if i >= args.len() {
                                eprintln!("seq: option requires an argument: -s");
                                return Err(1);
                            }
                            separator = args[i].clone();
                        } else {
                            separator = rest;
                        }
                        break;
                    }
                    _ => {
                        eprintln!("seq: invalid option: -{}", ch);
                        return Err(1);
                    }
                }
            }
            i += 1;
        } else {
            pos_args.push(arg);
            i += 1;
        }
    }

    let first: i64;
    let step: i64;
    let last: i64;

    match pos_args.len() {
        0 => {
            eprintln!("seq: usage: seq [FIRST [STEP]] LAST");
            return Err(1);
        }
        1 => {
            last = pos_args[0].parse().map_err(|_| {
                eprintln!("seq: invalid number");
                1
            })?;
            first = 1;
            step = 1;
        }
        2 => {
            first = pos_args[0].parse().map_err(|_| {
                eprintln!("seq: invalid number");
                1
            })?;
            last = pos_args[1].parse().map_err(|_| {
                eprintln!("seq: invalid number");
                1
            })?;
            step = 1;
        }
        3 => {
            first = pos_args[0].parse().map_err(|_| {
                eprintln!("seq: invalid number");
                1
            })?;
            step = pos_args[1].parse().map_err(|_| {
                eprintln!("seq: invalid number");
                1
            })?;
            last = pos_args[2].parse().map_err(|_| {
                eprintln!("seq: invalid number");
                1
            })?;
        }
        _ => {
            eprintln!("seq: too many arguments");
            return Err(1);
        }
    }

    if step == 0 {
        eprintln!("seq: step cannot be zero");
        return Err(1);
    }

    Ok((first, step, last, separator, equal_width))
}

fn generate_numbers(first: i64, step: i64, last: i64) -> Vec<i64> {
    let mut nums = Vec::new();
    if step > 0 {
        let mut i = first;
        while i <= last {
            nums.push(i);
            i += step;
        }
    } else {
        let mut i = first;
        while i >= last {
            nums.push(i);
            i += step;
        }
    }
    nums
}

fn format_numbers(nums: &[i64], separator: &str, equal_width: bool) -> String {
    if nums.is_empty() {
        return String::new();
    }

    let width = if equal_width {
        nums.iter()
            .map(|n| n.to_string().len())
            .max()
            .unwrap_or(1)
            .max(2)
    } else {
        0
    };

    let mut result = String::new();
    for (idx, n) in nums.iter().enumerate() {
        if idx > 0 {
            result.push_str(separator);
        }
        if equal_width {
            result.push_str(&format!("{:0width$}", n, width = width));
        } else {
            result.push_str(&n.to_string());
        }
    }
    result.push('\n');
    result
}

use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    match parse_args(args) {
        Ok((first, step, last, separator, equal_width)) => {
            if (step > 0 && first > last) || (step < 0 && first < last) {
                return 0;
            }
            let nums = generate_numbers(first, step, last);
            write!(stdout, "{}", format_numbers(&nums, &separator, equal_width)).ok();
            0
        }
        Err(code) => code,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_basic() {
        let args = vec!["5".to_string()];
        let (first, step, last, sep, w) = parse_args(&args).unwrap();
        assert_eq!(first, 1);
        assert_eq!(step, 1);
        assert_eq!(last, 5);
        assert_eq!(sep, "\n");
        assert!(!w);
    }

    #[test]
    fn test_parse_args_w_flag() {
        let args = vec!["-w".to_string(), "5".to_string()];
        let (first, step, last, _, w) = parse_args(&args).unwrap();
        assert_eq!(first, 1);
        assert_eq!(step, 1);
        assert_eq!(last, 5);
        assert!(w);
    }

    #[test]
    fn test_parse_args_s_flag() {
        let args = vec!["-s".to_string(), ",".to_string(), "5".to_string()];
        let (first, step, last, sep, _) = parse_args(&args).unwrap();
        assert_eq!(first, 1);
        assert_eq!(step, 1);
        assert_eq!(last, 5);
        assert_eq!(sep, ",");
    }

    #[test]
    fn test_format_numbers_default() {
        let nums = vec![1, 2, 3];
        assert_eq!(format_numbers(&nums, "\n", false), "1\n2\n3\n");
    }

    #[test]
    fn test_format_numbers_comma() {
        let nums = vec![1, 2, 3];
        assert_eq!(format_numbers(&nums, ",", false), "1,2,3\n");
    }

    #[test]
    fn test_format_numbers_equal_width() {
        let nums = vec![1, 10, 100];
        assert_eq!(format_numbers(&nums, "\n", true), "001\n010\n100\n");
    }

    #[test]
    fn test_format_numbers_empty() {
        assert_eq!(format_numbers(&[], ",", false), "");
    }

    #[test]
    fn test_format_numbers_short_equal_width_is_zero_padded() {
        assert_eq!(format_numbers(&[1, 2, 3], " ", true), "01 02 03\n");
    }

    #[test]
    fn test_generate_numbers_positive() {
        assert_eq!(generate_numbers(1, 1, 5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_generate_numbers_negative_step() {
        assert_eq!(generate_numbers(5, -1, 1), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_generate_numbers_step() {
        assert_eq!(generate_numbers(2, 3, 14), vec![2, 5, 8, 11, 14]);
    }

    #[test]
    fn test_seq_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_seq_single() {
        assert_eq!(run(&mut std::io::sink(), &["5".into()]), 0);
    }

    #[test]
    fn test_seq_first_last() {
        assert_eq!(run(&mut std::io::sink(), &["3".into(), "7".into()]), 0);
    }

    #[test]
    fn test_seq_full() {
        assert_eq!(
            run(&mut std::io::sink(), &["2".into(), "3".into(), "14".into()]),
            0
        );
    }

    #[test]
    fn test_seq_first_greater() {
        assert_eq!(run(&mut std::io::sink(), &["10".into(), "5".into()]), 0);
    }

    #[test]
    fn test_seq_negative_step() {
        assert_eq!(
            run(
                &mut std::io::sink(),
                &["10".into(), "-2".into(), "4".into()]
            ),
            0
        );
    }

    #[test]
    fn test_seq_step_zero() {
        assert_eq!(
            run(&mut std::io::sink(), &["1".into(), "0".into(), "5".into()]),
            1
        );
    }

    #[test]
    fn test_seq_invalid() {
        assert_eq!(run(&mut std::io::sink(), &["abc".into()]), 1);
    }

    #[test]
    fn test_seq_w_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-w".into(), "5".into()]), 0);
    }

    #[test]
    fn test_seq_with_separator() {
        assert_eq!(
            run(&mut std::io::sink(), &["-s".into(), ",".into(), "3".into()]),
            0
        );
    }
}
