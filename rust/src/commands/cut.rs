/// cut: remove sections from each line of files.
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Parse a list string like "1,3-5,7" into a sorted set of 1-based indices.
fn parse_list(list_str: &str) -> Option<Vec<usize>> {
    let mut indices = HashSet::new();
    for part in list_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            return None;
        }
        if let Some((start, end)) = part.split_once('-') {
            let s = start.trim().parse::<usize>().ok()?;
            let e = if end.trim().is_empty() {
                // "N-" means from N to end (we treat as just N)
                s
            } else {
                end.trim().parse::<usize>().ok()?
            };
            if s == 0 || e == 0 || s > e {
                return None;
            }
            for i in s..=e {
                indices.insert(i);
            }
        } else {
            let n = part.parse::<usize>().ok()?;
            if n == 0 {
                return None;
            }
            indices.insert(n);
        }
    }
    let mut sorted: Vec<usize> = indices.into_iter().collect();
    sorted.sort_unstable();
    Some(sorted)
}

/// Extract bytes at specified positions from a string
fn extract_bytes(line: &str, indices: &[usize]) -> String {
    let bytes = line.as_bytes();
    let mut result = Vec::new();
    for &idx in indices {
        if idx <= bytes.len() {
            result.push(bytes[idx - 1]);
        }
    }
    String::from_utf8_lossy(&result).into_owned()
}

/// Extract characters at specified positions from a string
fn extract_chars(line: &str, indices: &[usize]) -> String {
    let mut result = String::new();
    for &idx in indices {
        if let Some(c) = line.chars().nth(idx - 1) {
            result.push(c);
        }
    }
    result
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut delimiter: u8 = b'\t';
    let mut field_indices: Option<Vec<usize>> = None;
    let mut byte_indices: Option<Vec<usize>> = None;
    let mut char_indices: Option<Vec<usize>> = None;
    let mut suppress_non_matching = false;
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }

        // Handle bundled short options: -f1 or -d,
        if arg.starts_with('-') && arg.len() > 2 {
            let bytes = arg.as_bytes();
            let flag = bytes[1] as char;
            let val = &arg[2..];
            match flag {
                'b' => {
                    match parse_list(val) {
                        Some(indices) => byte_indices = Some(indices),
                        None => {
                            eprintln!("cut: invalid byte list: {}", val);
                            return 1;
                        }
                    }
                }
                'c' => {
                    match parse_list(val) {
                        Some(indices) => char_indices = Some(indices),
                        None => {
                            eprintln!("cut: invalid character list: {}", val);
                            return 1;
                        }
                    }
                }
                'd' => {
                    delimiter = val.as_bytes()[0];
                }
                'f' => {
                    match parse_list(val) {
                        Some(indices) => field_indices = Some(indices),
                        None => {
                            eprintln!("cut: invalid field list: {}", val);
                            return 1;
                        }
                    }
                }
                's' => {
                    suppress_non_matching = true;
                }
                _ => {
                    eprintln!("cut: invalid option: {}", arg);
                    return 1;
                }
            }
        } else if arg == "-b" {
            i += 1;
            if i >= args.len() {
                eprintln!("cut: option requires an argument: -b");
                return 1;
            }
            match parse_list(&args[i]) {
                Some(indices) => byte_indices = Some(indices),
                None => {
                    eprintln!("cut: invalid byte list: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-c" {
            i += 1;
            if i >= args.len() {
                eprintln!("cut: option requires an argument: -c");
                return 1;
            }
            match parse_list(&args[i]) {
                Some(indices) => char_indices = Some(indices),
                None => {
                    eprintln!("cut: invalid character list: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-d" {
            i += 1;
            if i >= args.len() {
                eprintln!("cut: option requires an argument: -d");
                return 1;
            }
            let d = args[i].as_bytes();
            if d.is_empty() {
                eprintln!("cut: invalid delimiter");
                return 1;
            }
            delimiter = d[0];
        } else if arg == "-f" {
            i += 1;
            if i >= args.len() {
                eprintln!("cut: option requires an argument: -f");
                return 1;
            }
            match parse_list(&args[i]) {
                Some(indices) => field_indices = Some(indices),
                None => {
                    eprintln!("cut: invalid field list: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-s" {
            suppress_non_matching = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("cut: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    // Check that exactly one of -f, -b, -c is specified
    let mut mode_count = 0;
    if field_indices.is_some() { mode_count += 1; }
    if byte_indices.is_some() { mode_count += 1; }
    if char_indices.is_some() { mode_count += 1; }
    
    if mode_count == 0 {
        eprintln!("cut: you must specify a list of fields, bytes, or characters");
        return 1;
    }
    
    if mode_count > 1 {
        eprintln!("cut: can only specify one of -f, -b, or -c");
        return 1;
    }

    if let Some(stdin) = crate::get_wasm_stdin() {
        if files.is_empty() {
            let mut reader = BufReader::new(stdin.as_bytes());
            process_reader(stdout, &mut reader, delimiter, field_indices.as_ref(), byte_indices.as_ref(), char_indices.as_ref(), suppress_non_matching);
        } else {
            for f in &files {
                match File::open(f) {
                    Ok(file) => {
                        let mut reader = BufReader::new(file);
                        process_reader(stdout, &mut reader, delimiter, field_indices.as_ref(), byte_indices.as_ref(), char_indices.as_ref(), suppress_non_matching);
                    }
                    Err(e) => {
                        eprintln!("cut: {}: {}", f, e);
                        return 1;
                    }
                }
            }
        }
        return 0;
    }

    if files.is_empty() {
        let mut reader = BufReader::new(io::stdin());
        process_reader(stdout, &mut reader, delimiter, field_indices.as_ref(), byte_indices.as_ref(), char_indices.as_ref(), suppress_non_matching);
    } else {
        for f in &files {
            match File::open(f) {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    process_reader(stdout, &mut reader, delimiter, field_indices.as_ref(), byte_indices.as_ref(), char_indices.as_ref(), suppress_non_matching);
                }
                Err(e) => {
                    eprintln!("cut: {}: {}", f, e);
                    return 1;
                }
            }
        }
    }

    0
}

fn process_reader(
    stdout: &mut dyn Write,
    reader: &mut dyn BufRead,
    delimiter: u8,
    field_indices: Option<&Vec<usize>>,
    byte_indices: Option<&Vec<usize>>,
    char_indices: Option<&Vec<usize>>,
    suppress_non_matching: bool,
) {
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        // Check if we should suppress this line due to -s flag
        if suppress_non_matching {
            if let Some(indices) = field_indices {
                // For field mode: suppress lines without delimiter
                let delim_char = delimiter as char;
                if !line.contains(delim_char) {
                    continue;
                }
            } else if let Some(indices) = byte_indices {
                // For byte mode: suppress lines shorter than max byte index
                let max_idx = indices.last().copied().unwrap_or(0);
                if line.as_bytes().len() < max_idx {
                    continue;
                }
            } else if let Some(indices) = char_indices {
                // For char mode: suppress lines shorter than max char index
                let max_idx = indices.last().copied().unwrap_or(0);
                if line.chars().count() < max_idx {
                    continue;
                }
            }
        }

        let output = if let Some(indices) = byte_indices {
            extract_bytes(&line, indices)
        } else if let Some(indices) = char_indices {
            extract_chars(&line, indices)
        } else if let Some(indices) = field_indices {
            extract_fields(&line, delimiter, indices)
        } else {
            String::new()
        };

        if !output.is_empty() {
            let _ = writeln!(stdout, "{}", output);
        }
    }
}

fn extract_fields(line: &str, delimiter: u8, indices: &[usize]) -> String {
    let fields: Vec<&str> = if delimiter == b'\t' {
        // Use standard split, but handle empty fields properly
        let mut result = Vec::new();
        let mut start = 0;
        for (pos, _) in line.match_indices('\t') {
            result.push(&line[start..pos]);
            start = pos + 1;
        }
        result.push(&line[start..]);
        result
    } else {
        let delim_char = delimiter as char;
        let mut result = Vec::new();
        let mut start = 0;
        for (pos, c) in line.char_indices() {
            if c == delim_char {
                result.push(&line[start..pos]);
                start = pos + c.len_utf8();
            }
        }
        result.push(&line[start..]);
        result
    };

    let selected: Vec<&str> = indices.iter()
        .filter_map(|&i| {
            if i > 0 && i <= fields.len() {
                Some(fields[i - 1])
            } else {
                None
            }
        })
        .collect();

    if selected.is_empty() {
        String::new()
    } else {
        let delim_str = String::from_utf8(vec![delimiter]).unwrap_or_default();
        selected.join(&delim_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_single() {
        assert_eq!(parse_list("1"), Some(vec![1]));
        assert_eq!(parse_list("3"), Some(vec![3]));
    }

    #[test]
    fn test_parse_list_range() {
        assert_eq!(parse_list("1-3"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_list_mixed() {
        let result = parse_list("1,3-5,7").unwrap();
        assert_eq!(result, vec![1, 3, 4, 5, 7]);
    }

    #[test]
    fn test_parse_list_invalid() {
        assert_eq!(parse_list("0"), None);
        assert_eq!(parse_list(""), None);
        assert_eq!(parse_list("abc"), None);
        assert_eq!(parse_list("1-0"), None);
    }

    #[test]
    fn test_cut_no_f_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-d".into(), ",".into()]), 1);
    }

    #[test]
    fn test_cut_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_cut_missing_d_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-d".into()]), 1);
    }

    #[test]
    fn test_cut_missing_f_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-f".into()]), 1);
    }

    #[test]
    fn test_cut_missing_b_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-b".into()]), 1);
    }

    #[test]
    fn test_cut_missing_c_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-c".into()]), 1);
    }

    #[test]
    fn test_cut_byte_mode() {
        let mut output = Vec::new();
        let args = vec!["-b".into()];
        // This should fail because no list is specified for -b
        let result = run(&mut output, &args);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_cut_char_mode() {
        let mut output = Vec::new();
        let args = vec!["-c".into()];
        // This should fail because no list is specified for -c
        let result = run(&mut output, &args);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_cut_suppress_flag() {
        let mut output = Vec::new();
        let args = vec!["-s".into()];
        // This should fail because no selection mode is specified
        let result = run(&mut output, &args);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_cut_mutually_exclusive_flags() {
        let mut output = Vec::new();
        let args = vec!["-f1".into(), "-b1".into()];
        let result = run(&mut output, &args);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_extract_bytes_basic() {
        assert_eq!(extract_bytes("hello", &[1, 3, 5]), "hlo");
        assert_eq!(extract_bytes("hello", &[2, 4]), "el");
        assert_eq!(extract_bytes("hello", &[10]), ""); // out of bounds
    }

    #[test]
    fn test_extract_chars_basic() {
        assert_eq!(extract_chars("hello", &[1, 3, 5]), "hlo");
        assert_eq!(extract_chars("hello", &[2, 4]), "el");
        assert_eq!(extract_chars("hello", &[10]), ""); // out of bounds
        assert_eq!(extract_chars("café", &[1, 2, 3, 4]), "café"); // unicode handling
    }

    #[test]
    fn test_extract_fields_basic() {
        assert_eq!(extract_fields("a,b,c", b',', &[1, 3]), "a,c");
        assert_eq!(extract_fields("a,b,c", b',', &[2]), "b");
        assert_eq!(extract_fields("a\tb\tc", b'\t', &[1, 2, 3]), "a\tb\tc");
    }
}
