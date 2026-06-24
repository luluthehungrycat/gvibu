/// grep: print lines matching a pattern.
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use crate::pwriteln;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

struct GrepConfig {
    recursive: bool,
    ignore_case: bool,
    invert: bool,
    count: bool,
    line_numbers: bool,
    files_with_matches: bool,
    fixed_strings: bool,
    extended_regexp: bool,
    word_regexp: bool,
    quiet: bool,
    only_matching: bool,
    no_messages: bool,
    after_context: usize,
    before_context: usize,
}

impl Default for GrepConfig {
    fn default() -> Self {
        Self {
            recursive: false,
            ignore_case: false,
            invert: false,
            count: false,
            line_numbers: false,
            files_with_matches: false,
            fixed_strings: false,
            extended_regexp: false,
            word_regexp: false,
            quiet: false,
            only_matching: false,
            no_messages: false,
            after_context: 0,
            before_context: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Matcher abstraction
// ---------------------------------------------------------------------------

enum Matcher {
    Regex(Regex),
    Literal {
        pattern: String,
        ignore_case: bool,
        whole_word: bool,
    },
}

impl Matcher {
    fn new(pattern: &str, cfg: &GrepConfig) -> Result<Self, String> {
        if cfg.fixed_strings {
            Ok(Matcher::Literal {
                pattern: pattern.to_string(),
                ignore_case: cfg.ignore_case,
                whole_word: cfg.word_regexp,
            })
        } else if cfg.word_regexp {
            let mut regex_str = String::new();
            if cfg.ignore_case {
                regex_str.push_str("(?i)");
            }
            regex_str.push_str(r"(?<![a-zA-Z0-9_])");
            regex_str.push_str(pattern);
            regex_str.push_str(r"(?![a-zA-Z0-9_])");
            Regex::new(&regex_str).map(Matcher::Regex).map_err(|e| e.to_string())
        } else {
            let mut regex_str = String::new();
            if cfg.ignore_case {
                regex_str.push_str("(?i)");
            }
            regex_str.push_str(pattern);
            Regex::new(&regex_str).map(Matcher::Regex).map_err(|e| e.to_string())
        }
    }

    fn is_match(&self, line: &str) -> bool {
        match self {
            Matcher::Regex(re) => re.is_match(line),
            Matcher::Literal { pattern, ignore_case, whole_word } => {
                if *ignore_case {
                    let line_lower = line.to_lowercase();
                    let pat_lower = pattern.to_lowercase();
                    find_literal(&line_lower, &pat_lower, *whole_word, line).is_some()
                } else {
                    find_literal(line, pattern.as_str(), *whole_word, line).is_some()
                }
            }
        }
    }

    fn find_matches<'a>(&self, line: &'a str) -> Vec<&'a str> {
        match self {
            Matcher::Regex(re) => re
                .find_iter(line)
                .map(|m| m.as_str())
                .filter(|s| !s.is_empty())
                .collect(),
            Matcher::Literal { pattern, ignore_case, whole_word } => {
                find_literal_all(line, pattern, *ignore_case, *whole_word)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Literal-match helpers
// ---------------------------------------------------------------------------

fn is_word_char_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn check_word_bounds(line: &str, start: usize, end: usize) -> bool {
    let bytes = line.as_bytes();
    let prev_word = start > 0 && bytes.get(start - 1).map_or(false, |&b| is_word_char_byte(b));
    let next_word = end < bytes.len() && bytes.get(end).map_or(false, |&b| is_word_char_byte(b));
    !prev_word && !next_word
}

/// Find the first occurrence of `pattern` in `search_in`.
fn find_literal(search_in: &str, pattern: &str, whole_word: bool, original: &str) -> Option<usize> {
    let mut offset = 0;
    while let Some(pos) = search_in[offset..].find(pattern) {
        let abs_pos = offset + pos;
        let end_pos = abs_pos + pattern.len();
        if !whole_word || check_word_bounds(original, abs_pos, end_pos) {
            return Some(abs_pos);
        }
        offset = abs_pos + 1;
    }
    None
}

/// Find all non-overlapping literal matches.
fn find_literal_all<'a>(
    line: &'a str,
    pattern: &str,
    ignore_case: bool,
    whole_word: bool,
) -> Vec<&'a str> {
    let mut result = Vec::new();
    let search_line = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };
    let search_pat = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let mut offset = 0;
    while let Some(pos) = search_line[offset..].find(&search_pat) {
        let abs_pos = offset + pos;
        let end_pos = abs_pos + pattern.len();
        if !whole_word || check_word_bounds(line, abs_pos, end_pos) {
            result.push(&line[abs_pos..end_pos]);
        }
        if end_pos > abs_pos {
            offset = end_pos;
        } else {
            offset = abs_pos + 1;
            if offset > line.len() {
                break;
            }
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Output helpers
// ---------------------------------------------------------------------------

fn write_line(
    w: &mut dyn Write,
    text: &str,
    filename: Option<&str>,
    line_num: Option<usize>,
) -> io::Result<()> {
    match (filename, line_num) {
        (Some(fname), Some(num)) => writeln!(w, "{}:{}:{}", fname, num, text),
        (Some(fname), None) => writeln!(w, "{}:{}", fname, text),
        (None, Some(num)) => writeln!(w, "{}:{}", num, text),
        (None, None) => writeln!(w, "{}", text),
    }
}

// ---------------------------------------------------------------------------
// Core reader
// ---------------------------------------------------------------------------

fn grep_reader_impl<R: BufRead>(
    w: &mut dyn Write,
    matcher: &Matcher,
    cfg: &GrepConfig,
    filename: Option<&str>,
    reader: R,
) -> i32 {
    let mut match_count = 0u64;
    let mut matched = false;
    let mut before_buf: VecDeque<(usize, String)> = VecDeque::new();
    let mut after_left: usize = 0;
    let mut is_first_group = true;
    let use_separator = cfg.before_context > 0 || cfg.after_context > 0;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => return 1,
        };

        let is_line_match = matcher.is_match(&line);
        let do_print = if cfg.invert { !is_line_match } else { is_line_match };

        if do_print {
            matched = true;
            if cfg.quiet {
                return 0;
            }

            if cfg.count {
                match_count += 1;
                before_buf.push_back((line_num, line));
                if cfg.before_context > 0 && before_buf.len() > cfg.before_context {
                    before_buf.pop_front();
                }
                continue;
            }

            if cfg.files_with_matches {
                if let Some(fname) = filename {
                    pwriteln!(w, "{}", fname);
                }
                return 0;
            }

            if !is_first_group && use_separator {
                pwriteln!(w, "--");
            }
            is_first_group = false;

            for (_, ctx_line) in before_buf.drain(..) {
                if let Err(_) = write_line(w, &ctx_line, filename, None) {
                    return 1;
                }
            }

            if cfg.only_matching {
                for m in matcher.find_matches(&line) {
                    if let Err(_) = write_line(w, m, filename, Some(line_num + 1)) {
                        return 1;
                    }
                }
            } else {
                let ln = if cfg.line_numbers { Some(line_num + 1) } else { None };
                if let Err(_) = write_line(w, &line, filename, ln) {
                    return 1;
                }
            }

            match_count += 1;
            after_left = cfg.after_context;
        } else if after_left > 0 {
            let ln = if cfg.line_numbers { Some(line_num + 1) } else { None };
            if let Err(_) = write_line(w, &line, filename, ln) {
                return 1;
            }
            after_left -= 1;
        }

        if cfg.before_context > 0 {
            if before_buf.len() >= cfg.before_context {
                before_buf.pop_front();
            }
            before_buf.push_back((line_num, line));
        }
    }

    if cfg.count && matched {
        if let Some(fname) = filename {
            pwriteln!(w, "{}:{}", fname, match_count);
        } else {
            pwriteln!(w, "{}", match_count);
        }
    }

    if matched { 0 } else { 1 }
}

// ---------------------------------------------------------------------------
// Directory walker
// ---------------------------------------------------------------------------

fn grep_dir_impl(
    w: &mut dyn Write,
    matcher: &Matcher,
    cfg: &GrepConfig,
    dir: &Path,
    show_filename: bool,
) -> i32 {
    let mut exit_code = 1;
    let entries = match std::fs::read_dir(dir) {
        Ok(d) => d,
        Err(e) => {
            if !cfg.no_messages {
                eprintln!("grep: {}: {}", dir.display(), e);
            }
            return 1;
        }
    };

    for entry in entries {
        match entry {
            Ok(e) => {
                let path = e.path();
                if path.is_dir() {
                    exit_code |= grep_dir_impl(w, matcher, cfg, &path, show_filename);
                } else if path.is_file() {
                    let filename = if show_filename {
                        Some(path.to_string_lossy().into_owned())
                    } else {
                        None
                    };
                    match File::open(&path) {
                        Ok(f) => {
                            let result = grep_reader_impl(
                                w,
                                matcher,
                                cfg,
                                filename.as_deref(),
                                BufReader::new(f),
                            );
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

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut cfg = GrepConfig::default();
    let mut pattern_str: Option<String> = None;
    let mut paths: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--ignore-case" => cfg.ignore_case = true,
            "-r" | "-R" | "--recursive" => cfg.recursive = true,
            "-v" | "--invert-match" => cfg.invert = true,
            "-c" | "--count" => cfg.count = true,
            "-n" | "--line-number" => cfg.line_numbers = true,
            "-l" | "--files-with-matches" => cfg.files_with_matches = true,
            "-E" | "--extended-regexp" => cfg.extended_regexp = true,
            "-F" | "--fixed-strings" => cfg.fixed_strings = true,
            "-w" | "--word-regexp" => cfg.word_regexp = true,
            "-q" | "--quiet" => cfg.quiet = true,
            "-o" | "--only-matching" => cfg.only_matching = true,
            "-s" | "--no-messages" => cfg.no_messages = true,
            "-A" | "--after-context" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("grep: option requires an argument: -A");
                    return 1;
                }
                match args[i].parse::<usize>() {
                    Ok(v) => cfg.after_context = v,
                    Err(_) => {
                        eprintln!("grep: invalid context length argument: {}", args[i]);
                        return 1;
                    }
                }
            }
            "-B" | "--before-context" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("grep: option requires an argument: -B");
                    return 1;
                }
                match args[i].parse::<usize>() {
                    Ok(v) => cfg.before_context = v,
                    Err(_) => {
                        eprintln!("grep: invalid context length argument: {}", args[i]);
                        return 1;
                    }
                }
            }
            "-C" | "--context" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("grep: option requires an argument: -C");
                    return 1;
                }
                match args[i].parse::<usize>() {
                    Ok(v) => {
                        cfg.after_context = v;
                        cfg.before_context = v;
                    }
                    Err(_) => {
                        eprintln!("grep: invalid context length argument: {}", args[i]);
                        return 1;
                    }
                }
            }
            "--" => break,
            s if s.starts_with('-') && s.len() > 1 => {
                // Handle bundled flags like -iv or -A5
                let chars: Vec<char> = s[1..].chars().collect();
                let mut j = 0;
                
                while j < chars.len() {
                    let c = chars[j];
                    
                    // Check for bundled context arguments like -A5, -B2, -C3
                    if (c == 'A' || c == 'B' || c == 'C') && j + 1 < chars.len() {
                        // Try to parse the number part
                        let mut num_chars = Vec::new();
                        let mut k = j + 1;
                        while k < chars.len() && chars[k].is_ascii_digit() {
                            num_chars.push(chars[k]);
                            k += 1;
                        }
                        
                        if !num_chars.is_empty() {
                            let num_str: String = num_chars.iter().collect();
                            match num_str.parse::<usize>() {
                                Ok(v) => {
                                    match c {
                                        'A' => cfg.after_context = v,
                                        'B' => cfg.before_context = v,
                                        'C' => {
                                            cfg.after_context = v;
                                            cfg.before_context = v;
                                        }
                                        _ => {}
                                    }
                                    j = k; // Skip the flag char and number
                                    continue;
                                }
                                Err(_) => {
                                    eprintln!("grep: invalid context length argument: {}", num_str);
                                    return 1;
                                }
                            }
                        }
                    }
                    
                    // Handle single character flags
                    match c {
                        'i' => cfg.ignore_case = true,
                        'r' | 'R' => cfg.recursive = true,
                        'v' => cfg.invert = true,
                        'c' => cfg.count = true,
                        'n' => cfg.line_numbers = true,
                        'l' => cfg.files_with_matches = true,
                        'E' => cfg.extended_regexp = true,
                        'F' => cfg.fixed_strings = true,
                        'w' => cfg.word_regexp = true,
                        'q' => cfg.quiet = true,
                        'o' => cfg.only_matching = true,
                        's' => cfg.no_messages = true,
                        '-' => {}
                        _ => {
                            eprintln!("grep: invalid option: -{}", c);
                            return 1;
                        }
                    }
                    j += 1;
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

    let matcher = match Matcher::new(&pattern, &cfg) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("grep: invalid pattern: {}", e);
            return 1;
        }
    };

    if paths.is_empty() && !cfg.recursive {
        return grep_reader_impl(w, &matcher, &cfg, None, io::stdin().lock());
    }

    let mut exit_code = 1;
    let show_filename = paths.len() > 1 || cfg.recursive;

    for path_str in &paths {
        let path = Path::new(path_str);

        if path.is_dir() {
            if cfg.recursive {
                exit_code |= grep_dir_impl(w, &matcher, &cfg, path, show_filename);
            } else {
                if !cfg.no_messages {
                    eprintln!("grep: {}: Is a directory", path_str);
                }
                exit_code = 1;
            }
        } else if path.is_file() || path_str == "-" {
            let filename = if path_str == "-" { None } else { Some(path_str.as_str()) };
            match File::open(path_str) {
                Ok(f) => {
                    let result = grep_reader_impl(w, &matcher, &cfg, filename, BufReader::new(f));
                    if result == 0 {
                        exit_code = 0;
                    }
                }
                Err(e) => {
                    if !cfg.no_messages {
                        eprintln!("grep: {}: {}", path_str, e);
                    }
                    exit_code = 1;
                }
            }
        } else {
            if !cfg.no_messages {
                eprintln!("grep: {}: No such file or directory", path_str);
            }
            exit_code = 1;
        }
    }

    exit_code
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
