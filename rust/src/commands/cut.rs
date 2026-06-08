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

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut delimiter: u8 = b'\t';
    let mut field_indices: Option<Vec<usize>> = None;
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg == "-d" {
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
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("cut: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if field_indices.is_none() {
        eprintln!("cut: you must specify a list of fields with -f");
        return 1;
    }

    let indices = field_indices.unwrap();
    let max_idx = *indices.last().unwrap_or(&1);

    if files.is_empty() {
        let mut reader = BufReader::new(io::stdin());
        process_reader(stdout, &mut reader, delimiter, &indices, max_idx);
    } else {
        for f in &files {
            match File::open(f) {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    process_reader(stdout, &mut reader, delimiter, &indices, max_idx);
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
    indices: &[usize],
    _max_idx: usize,
) {
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

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

        if !selected.is_empty() {
            let delim_str = String::from_utf8(vec![delimiter]).unwrap_or_default();
            let _ = writeln!(stdout, "{}", selected.join(&delim_str));
        }
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
}
