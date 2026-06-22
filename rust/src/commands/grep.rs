/// grep: print lines matching a pattern.
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use crate::pwriteln;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut ignore_case = false;
    let mut recursive = false;
    let mut invert = false;
    let mut count = false;
    let mut line_numbers = false;
    let mut files_with_matches = false;
    let mut pattern_str: Option<String> = None;
    let mut paths: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--ignore-case" => ignore_case = true,
            "-r" | "-R" | "--recursive" => recursive = true,
            "-v" | "--invert-match" => invert = true,
            "-c" | "--count" => count = true,
            "-n" | "--line-number" => line_numbers = true,
            "-l" | "--files-with-matches" => files_with_matches = true,
            "--" => { break; }
            s if s.starts_with('-') && s.len() > 1 => {
                // Handle bundled flags like -iv
                for c in s[1..].chars() {
                    match c {
                        'i' => ignore_case = true,
                        'r' | 'R' => recursive = true,
                        'v' => invert = true,
                        'c' => count = true,
                        'n' => line_numbers = true,
                        'l' => files_with_matches = true,
                        '-' => {} // end of options (--)
                        _ => {
                            eprintln!("grep: invalid option: -{}", c);
                            return 1;
                        }
                    }
                }
            }
            p => {
                if pattern_str.is_none() && !p.starts_with('-') {
                    pattern_str = Some(p.to_string());
                } else {
                    paths.push(p.to_string());
                }
            }
        }
        i += 1;
    }

    let pattern = match pattern_str {
        Some(p) => p,
        None => {
            eprintln!("grep: missing pattern");
            return 1;
        }
    };

    // Build regex
    let regex_pattern = if ignore_case {
        format!("(?i){}", pattern)
    } else {
        pattern.clone()
    };

    let re = match Regex::new(&regex_pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("grep: invalid pattern: {}", e);
            return 1;
        }
    };

    if paths.is_empty() && !recursive {
        // Read from stdin
        return grep_reader(w, &re, invert, count, line_numbers, files_with_matches, None, io::stdin().lock());
    }

    let mut exit_code = 1; // default: no match found
    let show_filename = paths.len() > 1 || recursive;

    for path_str in &paths {
        let path = Path::new(path_str);

        if path.is_dir() {
            if recursive {
                exit_code |= grep_dir(w, &re, invert, count, line_numbers, files_with_matches, path, show_filename);
            } else {
                eprintln!("grep: {}: Is a directory", path_str);
                exit_code = 1;
            }
        } else if path.is_file() || path_str == "-" {
            let filename = if path_str == "-" { None } else { Some(path_str.as_str()) };
            match File::open(path_str) {
                Ok(f) => {
                    let result = grep_reader(w, &re, invert, count, line_numbers, files_with_matches, filename, BufReader::new(f));
                    if result == 0 {
                        exit_code = 0;
                    }
                }
                Err(e) => {
                    eprintln!("grep: {}: {}", path_str, e);
                    exit_code = 1;
                }
            }
        } else {
            eprintln!("grep: {}: No such file or directory", path_str);
            exit_code = 1;
        }
    }

    exit_code
}

fn grep_dir(
    w: &mut dyn Write,
    re: &Regex,
    invert: bool,
    count: bool,
    line_numbers: bool,
    files_with_matches: bool,
    dir: &Path,
    show_filename: bool,
) -> i32 {
    let mut exit_code = 1;
    let entries = match std::fs::read_dir(dir) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("grep: {}: {}", dir.display(), e);
            return 1;
        }
    };

    for entry in entries {
        match entry {
            Ok(e) => {
                let path = e.path();
                if path.is_dir() {
                    exit_code |= grep_dir(w, re, invert, count, line_numbers, files_with_matches, &path, show_filename);
                } else if path.is_file() {
                    let filename = if show_filename { Some(path.to_string_lossy().into_owned()) } else { None };
                    match File::open(&path) {
                        Ok(f) => {
                            let result = grep_reader(w, re, invert, count, line_numbers, files_with_matches, filename.as_deref(), BufReader::new(f));
                            if result == 0 {
                                exit_code = 0;
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
    }

    exit_code
}

fn grep_reader<R: BufRead>(
    w: &mut dyn Write,
    re: &Regex,
    invert: bool,
    count: bool,
    line_numbers: bool,
    files_with_matches: bool,
    filename: Option<&str>,
    reader: R,
) -> i32 {
    let mut match_count = 0u64;
    let mut matched = false;

    for (line_num, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => return 1,
        };

        let is_match = re.is_match(&line);
        let print_line = if invert { !is_match } else { is_match };

        if print_line {
            matched = true;
            match_count += 1;

            if files_with_matches {
                if let Some(fname) = filename {
                    pwriteln!(w, "{}", fname);
                }
                return 0; // found a match, stop reading
            }

            if count {
                continue;
            }

            if let Some(fname) = filename {
                if line_numbers {
                    pwriteln!(w, "{}:{}:{}", fname, line_num + 1, line);
                } else {
                    pwriteln!(w, "{}:{}", fname, line);
                }
            } else {
                if line_numbers {
                    pwriteln!(w, "{}:{}", line_num + 1, line);
                } else {
                    pwriteln!(w, "{}", line);
                }
            }
        }
    }

    if count && matched {
        if let Some(fname) = filename {
            pwriteln!(w, "{}:{}", fname, match_count);
        } else {
            pwriteln!(w, "{}", match_count);
        }
    }

    if matched { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grep_missing_pattern() {
        let result = run(&mut std::io::sink(), &[]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_grep_no_match_stdin() {
        // With no stdin, pattern won't match
        let result = run(&mut std::io::sink(), &["xyz".into()]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_grep_invalid_option() {
        let result = run(&mut std::io::sink(), &["-x".into(), "pattern".into()]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_grep_invalid_pattern() {
        let result = run(&mut std::io::sink(), &["[invalid".into()]);
        assert_eq!(result, 1);
    }
}
