/// sort: sort lines of text files.
/// Supports: -r (reverse), -n (numeric), -V (version), -M (month),
/// -c (check), -u (unique), -f (fold case),
/// -k POS1[,POS2] (key sort with optional n/r modifiers)
use std::fs;
use std::io::{self, BufRead, Write};
use crate::pwriteln;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortMode {
    Lexical,
    Numeric,
    Version,
    Month,
}

fn lexical_cmp(a: &str, b: &str, fold_case: bool) -> std::cmp::Ordering {
    if fold_case {
        a.to_lowercase().cmp(&b.to_lowercase())
    } else {
        a.cmp(b)
    }
}

fn numeric_cmp(a: &str, b: &str, fold_case: bool) -> std::cmp::Ordering {
    match (numeric_prefix(a), numeric_prefix(b)) {
        (Some(na), Some(nb)) => na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => lexical_cmp(a, b, fold_case),
    }
}

fn month_number(s: &str) -> u8 {
    let prefix: String = s
        .trim_start()
        .chars()
        .take(3)
        .collect::<String>()
        .to_ascii_lowercase();
    match prefix.as_str() {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => 0,
    }
}

fn month_cmp(a: &str, b: &str, fold_case: bool) -> std::cmp::Ordering {
    let month_cmp = month_number(a).cmp(&month_number(b));
    if month_cmp == std::cmp::Ordering::Equal {
        lexical_cmp(a, b, fold_case)
    } else {
        month_cmp
    }
}

fn version_char_cmp(a: char, b: char) -> std::cmp::Ordering {
    match (a, b) {
        ('~', '~') => std::cmp::Ordering::Equal,
        ('~', _) => std::cmp::Ordering::Less,
        (_, '~') => std::cmp::Ordering::Greater,
        _ => a.cmp(&b),
    }
}

fn version_cmp(a: &str, b: &str, fold_case: bool) -> std::cmp::Ordering {
    let a_folded;
    let b_folded;
    let a = if fold_case {
        a_folded = a.to_lowercase();
        a_folded.as_str()
    } else {
        a
    };
    let b = if fold_case {
        b_folded = b.to_lowercase();
        b_folded.as_str()
    } else {
        b
    };
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut ai = 0;
    let mut bi = 0;

    while ai < a_chars.len() && bi < b_chars.len() {
        if a_chars[ai].is_ascii_digit() && b_chars[bi].is_ascii_digit() {
            let a_start = ai;
            let b_start = bi;
            while ai < a_chars.len() && a_chars[ai].is_ascii_digit() {
                ai += 1;
            }
            while bi < b_chars.len() && b_chars[bi].is_ascii_digit() {
                bi += 1;
            }

            let a_significant_start = (a_start..ai)
                .find(|&index| a_chars[index] != '0')
                .unwrap_or(ai.saturating_sub(1));
            let b_significant_start = (b_start..bi)
                .find(|&index| b_chars[index] != '0')
                .unwrap_or(bi.saturating_sub(1));
            let a_significant_len = ai - a_significant_start;
            let b_significant_len = bi - b_significant_start;
            if a_significant_len != b_significant_len {
                return a_significant_len.cmp(&b_significant_len);
            }
            for offset in 0..a_significant_len {
                let cmp = a_chars[a_significant_start + offset]
                    .cmp(&b_chars[b_significant_start + offset]);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }

            // Equal numeric values sort by the number of leading zeroes,
            // with the longer original run first (e.g. 01 before 1).
            let run_cmp = (ai - a_start).cmp(&(bi - b_start));
            if run_cmp != std::cmp::Ordering::Equal {
                return run_cmp.reverse();
            }
            continue;
        }

        let cmp = version_char_cmp(a_chars[ai], b_chars[bi]);
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }
        ai += 1;
        bi += 1;
    }

    if ai == a_chars.len() && bi == b_chars.len() {
        std::cmp::Ordering::Equal
    } else if ai == a_chars.len() {
        if b_chars[bi] == '~' {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    } else if a_chars[ai] == '~' {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn compare_values(a: &str, b: &str, mode: SortMode, fold_case: bool) -> std::cmp::Ordering {
    match mode {
        SortMode::Lexical => lexical_cmp(a, b, fold_case),
        SortMode::Numeric => numeric_cmp(a, b, fold_case),
        SortMode::Version => version_cmp(a, b, fold_case),
        SortMode::Month => month_cmp(a, b, fold_case),
    }
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
    if end_field.map_or(false, |ef| ef == start_field) {
        let field = fields[start_field];
        let start_char = (spec.char1.saturating_sub(1)).min(field.len());
        let limit = match spec.char2 {
            Some(c) if end_field.is_some() => c.min(field.len()),
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
                Some(c) => c.min(f.len()),
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
    let mut version_sort = false;
    let mut month_sort = false;
    let mut check = false;
    let mut unique = false;
    let mut fold_case = false;
    let mut key_specs: Vec<KeySpec> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" {
            files.extend(args[i + 1..].iter().cloned());
            break;
        }
        if arg == "-r" {
            reverse = true;
        } else if arg == "-n" {
            numeric = true;
        } else if arg == "-V" || arg == "--version-sort" {
            version_sort = true;
        } else if arg == "-M" || arg == "--month-sort" {
            month_sort = true;
        } else if arg == "-c"
            || arg == "--check"
            || arg == "--check=quiet"
            || arg == "--check=diagnose-first"
        {
            check = true;
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
        } else if arg.starts_with("--") {
            eprintln!("sort: invalid option: {}", arg);
            return 1;
        } else if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'r' => reverse = true,
                    'n' => numeric = true,
                    'V' => version_sort = true,
                    'M' => month_sort = true,
                    'c' => check = true,
                    'u' => unique = true,
                    'f' => fold_case = true,
                    'k' => {
                        // Combined like -k2 (no space before value)
                        let spec = &arg[arg.find('k').expect("sort -k: 'k' not found in arg") + 1..];
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
                        break;
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
    let global_mode = if version_sort {
        SortMode::Version
    } else if month_sort {
        SortMode::Month
    } else if numeric {
        SortMode::Numeric
    } else {
        SortMode::Lexical
    };

    let make_sort_key = |line: &str| -> Vec<String> {
        key_specs.iter().map(|ks| extract_key(line, ks)).collect()
    };
    let mut entries: Vec<(Vec<String>, String)> = lines
        .into_iter()
        .map(|line| {
            let keys = make_sort_key(&line);
            (keys, line)
        })
        .collect();

    let compare_entries = |a: &(Vec<String>, String), b: &(Vec<String>, String)| {
        for (index, ks) in key_specs.iter().enumerate() {
            let mode = if ks.numeric {
                SortMode::Numeric
            } else {
                global_mode
            };
            let mut cmp = compare_values(&a.0[index], &b.0[index], mode, fold_case);
            if cmp != std::cmp::Ordering::Equal {
                if ks.reverse || reverse {
                    cmp = cmp.reverse();
                }
                return cmp;
            }
        }

        let mut cmp = compare_values(&a.1, &b.1, global_mode, fold_case);
        if reverse {
            cmp = cmp.reverse();
        }
        cmp
    };

    if check {
        for pair in entries.windows(2) {
            if compare_entries(&pair[0], &pair[1]) == std::cmp::Ordering::Greater {
                return 1;
            }
        }
        return 0;
    }

    entries.sort_by(|a, b| compare_entries(a, b));
    let mut result: Vec<String> = entries.into_iter().map(|(_, line)| line).collect();

    if unique {
        result.dedup();
    }

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
        assert_eq!(run(&mut std::io::sink(), &["-k".into(), "2".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_sort_key_invalid() {
        assert_eq!(run(&mut std::io::sink(), &["-k".into(), "abc".into()]), 1);
    }

    #[test]
    fn test_sort_key_missing_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-k".into()]), 1);
    }
    fn run_with_input(input: &str, options: &[&str]) -> (i32, String) {
        let path = std::env::temp_dir().join(format!(
            "gvibu-sort-test-{}-{}",
            std::process::id(),
            options.join("-").replace('/', "_")
        ));
        std::fs::write(&path, input).unwrap();

        let mut args: Vec<String> = options.iter().map(|arg| (*arg).to_string()).collect();
        args.push(path.to_string_lossy().into_owned());
        let mut output = Vec::new();
        let status = run(&mut output, &args);

        std::fs::remove_file(path).unwrap();
        (status, String::from_utf8(output).unwrap())
    }

    #[test]
    fn test_version_sort_flag() {
        let (status, output) = run_with_input("pkg10\npkg2\npkg1\n", &["-V"]);
        assert_eq!(status, 0);
        assert_eq!(output, "pkg1\npkg2\npkg10\n");
    }

    #[test]
    fn test_month_sort_long_flag() {
        let (status, output) = run_with_input("Dec\nfoo\nJan\nFeb\n", &["--month-sort"]);
        assert_eq!(status, 0);
        assert_eq!(output, "foo\nJan\nFeb\nDec\n");
    }

    #[test]
    fn test_check_flag_reports_disorder_without_output() {
        let (status, output) = run_with_input("a\nb\n", &["-c"]);
        assert_eq!(status, 0);
        assert!(output.is_empty());

        let (status, output) = run_with_input("b\na\n", &["--check"]);
        assert_ne!(status, 0);
        assert!(output.is_empty());
    }
}
