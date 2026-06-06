/// wc: print newline, word, byte, and character counts.
use std::fs;
use std::io::{self, Read};

fn count_data(data: &str) -> (usize, usize, usize, usize) {
    let lines = data.matches('\n').count();
    let words = if data.is_empty() {
        0
    } else {
        data.split_whitespace().count()
    };
    let bytes = data.len();
    let chars = data.chars().count();
    (lines, words, bytes, chars)
}

pub fn run(args: &[String]) -> i32 {
    let mut flag_l = false;
    let mut flag_w = false;
    let mut flag_c = false;
    let mut flag_m = false;

    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'l' => flag_l = true,
                    'w' => flag_w = true,
                    'c' => flag_c = true,
                    'm' => flag_m = true,
                    _ => {
                        eprintln!("wc: invalid option: -{}", ch);
                        return 1;
                    }
                }
            }
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    // Default: all four
    if !flag_l && !flag_w && !flag_c && !flag_m {
        flag_l = true;
        flag_w = true;
        flag_c = true;
        flag_m = true;
    }

    let fmt = |lines: usize, words: usize, bytes: usize, chars: usize, name: &str| -> String {
        let mut parts = Vec::new();
        if flag_l {
            parts.push(format!("{:>7}", lines));
        }
        if flag_w {
            parts.push(format!("{:>7}", words));
        }
        if flag_c {
            parts.push(format!("{:>7}", bytes));
        }
        if flag_m {
            parts.push(format!("{:>7}", chars));
        }
        if !name.is_empty() {
            parts.push(name.to_string());
        }
        parts.join(" ")
    };

    let mut exit_code = 0;
    let mut total_l = 0usize;
    let mut total_w = 0usize;
    let mut total_c = 0usize;
    let mut total_m = 0usize;

    if files.is_empty() {
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buf) {
            eprintln!("wc: stdin: {}", e);
            return 1;
        }
        let (l, w, c, m) = count_data(&buf);
        println!("{}", fmt(l, w, c, m, ""));
        return 0;
    }

    for fname in &files {
        let data = match fs::read_to_string(fname) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("wc: {}: {}", fname, e);
                exit_code = 1;
                continue;
            }
        };

        let (l, w, c, m) = count_data(&data);
        total_l += l;
        total_w += w;
        total_c += c;
        total_m += m;
        println!("{}", fmt(l, w, c, m, fname));
    }

    if files.len() > 1 {
        println!("{}", fmt(total_l, total_w, total_c, total_m, "total"));
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_data_empty() {
        assert_eq!(count_data(""), (0, 0, 0, 0));
    }

    #[test]
    fn test_count_data_basic() {
        assert_eq!(count_data("hello world\n"), (1, 2, 12, 12));
    }

    #[test]
    fn test_count_data_multiline() {
        assert_eq!(count_data("a\nb\nc\n"), (3, 3, 6, 6));
    }

    #[test]
    fn test_count_data_utf8() {
        // "héllo wörld\n" is 13 bytes but 12 chars
        let (l, w, c, m) = count_data("héllo wörld\n");
        assert_eq!(l, 1);
        assert_eq!(w, 2);
        assert_eq!(c, 13);
        assert_eq!(m, 12);
    }

    #[test]
    fn test_invalid_option() {
        assert_eq!(run(&["-x".into(), "/dev/null".into()]), 1);
    }

    #[test]
    fn test_valid_options() {
        assert_eq!(run(&["-l".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&["-w".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&["-c".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&["-m".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_combined_options() {
        assert_eq!(run(&["-lw".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&["-lwm".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&["-lwcm".into(), "/dev/null".into()]), 0);
    }
}
