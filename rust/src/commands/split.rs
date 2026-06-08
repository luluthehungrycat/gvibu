/// split: split a file into pieces.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut lines: i64 = -1; // default 1000 lines
    let mut bytes: i64 = -1;
    let mut numeric = false;
    let mut suffix_len: usize = 2;
    let mut files: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        if arg == "--" { i += 1; break; }
        if arg == "-d" {
            numeric = true;
        } else if arg == "-a" {
            i += 1;
            if i >= args.len() {
                eprintln!("split: option requires an argument: -a");
                return 1;
            }
            match args[i].parse::<usize>() {
                Ok(n) if n >= 1 => suffix_len = n,
                _ => {
                    eprintln!("split: invalid suffix length: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-l" {
            i += 1;
            if i >= args.len() {
                eprintln!("split: option requires an argument: -l");
                return 1;
            }
            match args[i].parse::<i64>() {
                Ok(n) if n > 0 => lines = n,
                _ => {
                    eprintln!("split: invalid number of lines: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-b" {
            i += 1;
            if i >= args.len() {
                eprintln!("split: option requires an argument: -b");
                return 1;
            }
            match parse_size(&args[i]) {
                Ok(n) if n > 0 => bytes = n,
                _ => {
                    eprintln!("split: invalid number of bytes: {}", args[i]);
                    return 1;
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("split: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg);
        }
        i += 1;
    }

    if lines == -1 && bytes == -1 {
        lines = 1000;
    }

    let input_file = if files.is_empty() { None } else { Some(files[0]) };
    let prefix = if files.len() > 1 { files[1] } else { "x" };

    let reader: Box<dyn BufRead> = match input_file {
        Some(f) if f != "-" => {
            match File::open(f) {
                Ok(file) => Box::new(BufReader::new(file)),
                Err(e) => {
                    eprintln!("split: cannot open '{}': {}", f, e);
                    return 1;
                }
            }
        }
        _ => Box::new(BufReader::new(io::stdin())),
    };

    if bytes > 0 {
        split_bytes(reader, prefix, bytes, numeric, suffix_len)
    } else {
        split_lines(reader, prefix, lines, numeric, suffix_len)
    }
}

fn parse_size(s: &str) -> Result<i64, String> {
    let s = s.trim();
    let (num_part, multiplier) = if let Some(suffix) = s.chars().last() {
        let mult = match suffix {
            'b' => 512,
            'k' => 1024,
            'm' => 1024 * 1024,
            'g' => 1024 * 1024 * 1024,
            _ => {
                return s.parse::<i64>().map_err(|_| format!("invalid size: {}", s));
            }
        };
        (&s[..s.len() - 1], mult)
    } else {
        return Err("empty size".to_string());
    };
    let num: i64 = num_part.parse().map_err(|_| format!("invalid size: {}", s))?;
    Ok(num * multiplier)
}

fn split_lines<R: BufRead>(
    reader: R,
    prefix: &str,
    max_lines: i64,
    numeric: bool,
    suffix_len: usize,
) -> i32 {
    let mut file_num = 0u64;
    let mut line_count = 0i64;
    let mut current_file: Option<File> = None;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line_count >= max_lines {
            current_file = None;
            line_count = 0;
            file_num += 1;
        }

        if current_file.is_none() {
            let name = make_suffix(prefix, file_num, numeric, suffix_len);
            match File::create(&name) {
                Ok(f) => current_file = Some(f),
                Err(e) => {
                    eprintln!("split: cannot create '{}': {}", name, e);
                    return 1;
                }
            }
        }

        if let Some(ref f) = current_file {
            let mut f = f;
            if writeln!(f, "{}", line).is_err() {
                return 1;
            }
        }
        line_count += 1;
    }

    0
}

fn split_bytes<R: BufRead>(
    reader: R,
    prefix: &str,
    max_bytes: i64,
    numeric: bool,
    suffix_len: usize,
) -> i32 {
    let mut file_num = 0u64;
    let mut byte_count = 0i64;
    let mut current_file: Option<File> = None;

    for byte in reader.bytes() {
        let byte = match byte {
            Ok(b) => b,
            Err(_) => break,
        };

        if byte_count >= max_bytes {
            current_file = None;
            byte_count = 0;
            file_num += 1;
        }

        if current_file.is_none() {
            let name = make_suffix(prefix, file_num, numeric, suffix_len);
            match File::create(&name) {
                Ok(f) => current_file = Some(f),
                Err(e) => {
                    eprintln!("split: cannot create '{}': {}", name, e);
                    return 1;
                }
            }
        }

        if let Some(ref f) = current_file {
            let mut f = f;
            use std::io::Write;
            if f.write_all(&[byte]).is_err() {
                return 1;
            }
        }
        byte_count += 1;
    }

    0
}

fn make_suffix(prefix: &str, num: u64, numeric: bool, len: usize) -> String {
    if numeric {
        format!("{}{:0width$}", prefix, num, width = len)
    } else {
        // Alphabetic suffix: aa, ab, ..., zz, aaa, aab, ...
        let mut n = num;
        let mut suffix = String::new();
        loop {
            let rem = (n % 26) as u8;
            suffix.push((b'a' + rem) as char);
            n /= 26;
            if n == 0 {
                break;
            }
        }
        while suffix.len() < len {
            suffix.push('a');
        }
        let suffix: String = suffix.chars().rev().collect();
        format!("{}{}", prefix, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }

    #[test]
    fn test_split_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_split_missing_a_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-a".into()]), 1);
    }

    #[test]
    fn test_split_invalid_a_value() {
        assert_eq!(run(&mut std::io::sink(), &["-a".into(), "0".into()]), 1);
    }

    #[test]
    fn test_split_missing_l_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-l".into()]), 1);
    }

    #[test]
    fn test_split_invalid_lines() {
        assert_eq!(run(&mut std::io::sink(), &["-l".into(), "0".into()]), 1);
    }

    #[test]
    fn test_make_suffix_alpha() {
        assert_eq!(make_suffix("x", 0, false, 2), "xaa");
        assert_eq!(make_suffix("x", 1, false, 2), "xab");
        assert_eq!(make_suffix("x", 25, false, 2), "xaz");
        assert_eq!(make_suffix("x", 26, false, 2), "xba");
    }

    #[test]
    fn test_make_suffix_numeric() {
        assert_eq!(make_suffix("x", 0, true, 2), "x00");
        assert_eq!(make_suffix("x", 1, true, 2), "x01");
        assert_eq!(make_suffix("x", 99, true, 2), "x99");
    }
}
