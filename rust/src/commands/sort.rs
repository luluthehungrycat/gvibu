/// sort: sort lines of text files.
/// Supports: -r (reverse), -n (numeric), -u (unique), -f (fold case),
/// -k POS1[,POS2] (key sort with optional n/r modifiers)
use std::fs;
use std::io::{self, BufRead, Write};

fn read_lines(files: &[String]) -> Vec<String> {
    let mut lines = Vec::new();

    if files.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => lines.push(l),
                Err(_) => break,
            }
        }
    } else {
        for fname in files {
            if fname == "-" {
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    match line {
                        Ok(l) => lines.push(l),
                        Err(_) => break,
                    }
                }
            } else {
                match fs::File::open(fname) {
                    Ok(file) => {
                        let reader = io::BufReader::new(file);
                        for line in reader.lines() {
                            match line {
                                Ok(l) => lines.push(l),
                                Err(_) => break,
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("sort: {}: {}", fname, e);
                    }
                }
            }
        }
    }

    lines
}

fn numeric_prefix(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    if i < chars.len() && chars[i] == '-' {
        i += 1;
    }
    while i < chars.len() && chars[i].is_ascii_digit() {
        i += 1;
    }
    if i < chars.len() && chars[i] == '.' {
        i += 1;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i == 0 {
        return None;
    }
    s[..i].parse::<f64>().ok()
}

/// A key specification for -k sorting.
#[derive(Clone, Debug)]
struct KeySpec {
    field1: usize,          // 1-indexed start field
    char1: usize,           // 1-indexed start character within field
    field2: Option<usize>,  // 1-indexed end field (None = end of line)
    char2: Option<usize>,   // 1-indexed end character within end field
    numeric: bool,          // n modifier
    reverse: bool,          // r modifier
}

/// Parse a key spec string like "2", "2,4", "2.3", "2.3,4.5", "2n", "2nr"
fn parse_key_spec(spec: &str) -> Option<KeySpec> {
    // Separate the spec into the key part and trailing modifiers
    let mut spec_str = spec;
    let mut numeric = false;
    let mut reverse = false;

    // Extract trailing modifier characters (n, r from the end)
    while let Some(ch) = spec_str.chars().last() {
        match ch {
            'n' => {
                numeric = true;
                spec_str = &spec_str[..spec_str.len() - 1];
            }
            'r' => {
                reverse = true;
                spec_str = &spec_str[..spec_str.len() - 1];
            }
            _ => break,
        }
    }

    // Now spec_str is like "2", "2,4", "2.3", "2.3,4.5"
    let parts: Vec<&str> = spec_str.split(',').collect();
    if parts.is_empty() || parts[0].is_empty() {
        return None;
    }

    // Parse POS1: field[.char]
    let pos1_parts: Vec<&str> = parts[0].split('.').collect();
    let field1: usize = pos1_parts[0].parse().ok()?;
    if field1 < 1 {
        return None;
    }
    let char1: usize = if pos1_parts.len() > 1 {
        pos1_parts[1].parse().ok()?
    } else {
        1
    };

    // Parse optional POS2
    let (field2, char2) = if parts.len() > 1 && !parts[1].is_empty() {
        let pos2_parts: Vec<&str> = parts[1].split('.').collect();
        let f2: usize = pos2_parts[0].parse().ok()?;
        if f2 < 1 {
            return None;
        }
        let c2: usize = if pos2_parts.len() > 1 {
            pos2_parts[1].parse().ok()?
        } else {
            1
        };
        (Some(f2), Some(c2))
    } else {
        (None, None)
    };

    Some(KeySpec {
        field1,
        char1,
        field2,
        char2,
        numeric,
        reverse,
    })
}

/// Split a line into whitespace-separated fields (like GNU sort default).
fn split_fields(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

/// Extract the key portion of a line according to a KeySpec.
fn extract_key(line: &str, spec: &KeySpec) -> String {
    let fields = split_fields(line);

    let start_field = (spec.field1.saturating_sub(1)).min(fields.len().saturating_sub(1));
    let end_field = match spec.field2 {
        Some(f) => Some(f.saturating_sub(1).min(fields.len().saturating_sub(1))),
        None => None,
    };

    if fields.is_empty() || start_field >= fields.len() {
        return String::new();
    }

    // Single field case
    if end_field.map_or(true, |ef| ef == start_field) {
        let field = fields[start_field];
        let start_char = (spec.char1.saturating_sub(1)).min(field.len());
        let limit = match spec.char2 {
            Some(c) if end_field.is_some() => c.saturating_sub(1).min(field.len()),
            _ => field.len(),
        };
        if start_char >= limit {
            return String::new();
        }
        return field[start_char..limit].to_string();
    }

    // Multiple field case
    let end = end_field.unwrap_or(fields.len() - 1);
    let mut out = String::new();
    for (idx, f) in fields.iter().enumerate().skip(start_field).take(end - start_field + 1) {
        if idx > start_field {
            out.push(' ');
        }
        if idx == start_field {
            let start_char = (spec.char1.saturating_sub(1)).min(f.len());
            out.push_str(&f[start_char..]);
        } else if idx == end {
            let limit = match spec.char2 {
                Some(c) => c.saturating_sub(1).min(f.len()),
                None => f.len(),
            };
            out.push_str(&f[..limit]);
        } else {
            out.push_str(f);
        }
    }
    out
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut reverse = false;
    let mut numeric = false;
    let mut unique = false;
    let mut fold_case = false;
    let mut key_specs: Vec<KeySpec> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg == "-r" {
            reverse = true;
        } else if arg == "-n" {
            numeric = true;
        } else if arg == "-u" {
            unique = true;
        } else if arg == "-f" {
            fold_case = true;
        } else if arg == "-k" {
            i += 1;
            if i >= args.len() {
                eprintln!("sort: option requires an argument: -k");
                return 1;
            }
            match parse_key_spec(&args[i]) {
                Some(ks) => key_specs.push(ks),
                None => {
                    eprintln!("sort: invalid key specification: {}", args[i]);
                    return 1;
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'r' => reverse = true,
                    'n' => numeric = true,
                    'u' => unique = true,
                    'f' => fold_case = true,
                    'k' => {
                        // Combined like -k2 (no space before value)
                        let spec = &arg[arg.find('k').unwrap() + 1..];
                        if spec.is_empty() {
                            eprintln!("sort: option requires an argument: -k");
                            return 1;
                        }
                        match parse_key_spec(spec) {
                            Some(ks) => key_specs.push(ks),
                            None => {
                                eprintln!("sort: invalid key specification: {}", spec);
                                return 1;
                            }
                        }
                        break; // -k is the last meaningful char in combined flags
                    }
                    _ => {
                        eprintln!("sort: invalid option: -{}", ch);
                        return 1;
                    }
                }
            }
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let lines = read_lines(&files);

    // Build the sort key for a line: (key_values..., whole_line)
    let make_sort_key = |line: &str| -> Vec<String> {
        let mut keys = Vec::new();
        for ks in &key_specs {
            keys.push(extract_key(line, ks));
        }
        keys
    };

    let mut sorted: Vec<(Vec<String>, String)> = lines
        .into_iter()
        .map(|line| {
            let keys = make_sort_key(&line);
            (keys, line)
        })
        .collect();

    sorted.sort_by(|a, b| {
        for (i, ks) in key_specs.iter().enumerate() {
            let a_key = &a.0[i];
            let b_key = &b.0[i];

            let use_numeric = ks.numeric || numeric;
            let use_reverse = ks.reverse || reverse;

            let cmp = if use_numeric {
                let an = numeric_prefix(a_key);
                let bn = numeric_prefix(b_key);
                match (an, bn) {
                    (Some(na), Some(nb)) => na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => {
                        if fold_case {
                            a_key.to_lowercase().cmp(&b_key.to_lowercase())
                        } else {
                            a_key.cmp(b_key)
                        }
                    }
                }
            } else {
                if fold_case {
                    a_key.to_lowercase().cmp(&b_key.to_lowercase())
                } else {
                    a_key.cmp(b_key)
                }
            };

            if cmp != std::cmp::Ordering::Equal {
                if use_reverse {
                    return cmp.reverse();
                } else {
                    return cmp;
                }
            }
        }

        // Fall back to whole line comparison
        let cmp = if fold_case {
            a.1.to_lowercase().cmp(&b.1.to_lowercase())
        } else {
            a.1.cmp(&b.1)
        };
        if reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });

    // Extract sorted lines
    let mut result: Vec<String> = sorted.into_iter().map(|(_, line)| line).collect();

    // Unique: remove consecutive duplicates
    if unique {
        result.dedup();
    }

    // Output
    for line in &result {
        pwriteln!(w, "{}", line);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_prefix() {
        assert_eq!(numeric_prefix("42"), Some(42.0));
        assert_eq!(numeric_prefix("3.14"), Some(3.14));
        assert_eq!(numeric_prefix("-7"), Some(-7.0));
        assert_eq!(numeric_prefix("abc"), None);
        assert_eq!(numeric_prefix(""), None);
        assert_eq!(numeric_prefix("42abc"), Some(42.0));
    }

    #[test]
    fn test_parse_key_spec_simple() {
        let ks = parse_key_spec("2").unwrap();
        assert_eq!(ks.field1, 2);
        assert_eq!(ks.char1, 1);
        assert!(ks.field2.is_none());
        assert!(!ks.numeric);
        assert!(!ks.reverse);
    }

    #[test]
    fn test_parse_key_spec_range() {
        let ks = parse_key_spec("2,4").unwrap();
        assert_eq!(ks.field1, 2);
        assert_eq!(ks.field2, Some(4));
    }

    #[test]
    fn test_parse_key_spec_char() {
        let ks = parse_key_spec("2.3").unwrap();
        assert_eq!(ks.field1, 2);
        assert_eq!(ks.char1, 3);
    }

    #[test]
    fn test_parse_key_spec_modifiers() {
        let ks = parse_key_spec("2n").unwrap();
        assert_eq!(ks.field1, 2);
        assert!(ks.numeric);
    }

    #[test]
    fn test_parse_key_spec_both_mods() {
        let ks = parse_key_spec("2nr").unwrap();
        assert_eq!(ks.field1, 2);
        assert!(ks.numeric);
        assert!(ks.reverse);
    }

    #[test]
    fn test_parse_key_spec_full() {
        let ks = parse_key_spec("2.3,4.5n").unwrap();
        assert_eq!(ks.field1, 2);
        assert_eq!(ks.char1, 3);
        assert_eq!(ks.field2, Some(4));
        assert_eq!(ks.char2, Some(5));
        assert!(ks.numeric);
    }

    #[test]
    fn test_parse_key_spec_invalid() {
        assert!(parse_key_spec("").is_none());
        assert!(parse_key_spec("a").is_none());
    }

    #[test]
    fn test_extract_key_simple() {
        let ks = KeySpec { field1: 2, char1: 1, field2: None, char2: None, numeric: false, reverse: false };
        assert_eq!(extract_key("a b c d", &ks), "b c d");
    }

    #[test]
    fn test_extract_key_single_field() {
        let ks = KeySpec { field1: 2, char1: 1, field2: Some(2), char2: None, numeric: false, reverse: false };
        assert_eq!(extract_key("a b c d", &ks), "b");
    }

    #[test]
    fn test_extract_key_range() {
        let ks = KeySpec { field1: 1, char1: 1, field2: Some(2), char2: None, numeric: false, reverse: false };
        assert_eq!(extract_key("a b c", &ks), "a b");
    }

    #[test]
    fn test_extract_key_char_offset() {
        let ks = KeySpec { field1: 1, char1: 2, field2: Some(1), char2: Some(4), numeric: false, reverse: false };
        assert_eq!(extract_key("hello", &ks), "ell");
    }

    #[test]
    fn test_sort_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_sort_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_sort_combined_flags() {
        assert_eq!(run(&mut std::io::sink(), &["-rn".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_sort_empty_input() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }

    #[test]
    fn test_sort_no_flag() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_sort_key_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-k", "2", "/dev/null".into()]), 0);
    }

    #[test]
    fn test_sort_key_invalid() {
        assert_eq!(run(&mut std::io::sink(), &["-k", "abc"]), 1);
    }

    #[test]
    fn test_sort_key_missing_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-k"]), 1);
    }
}
