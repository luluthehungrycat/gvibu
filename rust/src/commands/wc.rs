/// wc: print newline, word, byte, character, and max-line-length counts.
/// Fully streaming: O(1) memory regardless of file size.
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

/// Stream-process byte reader counting lines, words, bytes, chars, max line length.
/// Char counting works on any valid UTF-8 by counting non-continuation bytes.
fn count_stream<R: Read>(reader: &mut R) -> io::Result<(usize, usize, usize, usize, usize)> {
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut bytes = 0usize;
    let mut chars = 0usize;
    let mut max_line: usize = 0;

    let mut buf = [0u8; 8192];
    let mut in_word = false;
    let mut current_line_len: usize = 0;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        for &b in &buf[..n] {
            bytes += 1;
            // Count chars: any byte that is not a UTF-8 continuation byte (0x80..0xBF)
            if b & 0xC0 != 0x80 {
                chars += 1;
            }

            match b {
                b'\n' => {
                    lines += 1;
                    max_line = max_line.max(current_line_len);
                    current_line_len = 0;
                    in_word = false;
                }
                b' ' | b'\t' | b'\r' => {
                    current_line_len += 1;
                    in_word = false;
                }
                _ => {
                    current_line_len += 1;
                    if !in_word {
                        words += 1;
                        in_word = true;
                    }
                }
            }
        }
    }

    // Handle last line without trailing newline
    max_line = max_line.max(current_line_len);

    Ok((lines, words, bytes, chars, max_line))
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut flag_l = false;
    let mut flag_w = false;
    let mut flag_c = false;
    let mut flag_m = false;
    let mut flag_L = false;

    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'l' => flag_l = true,
                    'w' => flag_w = true,
                    'c' => flag_c = true,
                    'm' => flag_m = true,
                    'L' => flag_L = true,
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

    // Default: l, w, c, m (not L)
    if !flag_l && !flag_w && !flag_c && !flag_m && !flag_L {
        flag_l = true;
        flag_w = true;
        flag_c = true;
        flag_m = true;
    }

    let fmt = |lines: usize, words: usize, bytes: usize, chars: usize, max_line: usize, name: &str| -> String {
        let mut parts = Vec::new();
        if flag_l { parts.push(format!("{:>7}", lines)); }
        if flag_w { parts.push(format!("{:>7}", words)); }
        if flag_c { parts.push(format!("{:>7}", bytes)); }
        if flag_m { parts.push(format!("{:>7}", chars)); }
        if flag_L { parts.push(format!("{:>7}", max_line)); }
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
    let mut total_L = 0usize;

    if files.is_empty() {
        let mut stdin = io::stdin().lock();
        match count_stream(&mut stdin) {
            Ok((l, w, c, m, max_line)) => {
                let _ = pwriteln!(w, "{}", fmt(l, w, c, m, max_line, ""));
            }
            Err(e) => {
                eprintln!("wc: stdin: {}", e);
                return 1;
            }
        }
        return 0;
    }

    for fname in &files {
        if fname == "-" {
            let mut stdin = io::stdin().lock();
            match count_stream(&mut stdin) {
                Ok((l, w, c, m, max_line)) => {
                    total_l += l;
                    total_w += w;
                    total_c += c;
                    total_m += m;
                    total_L = total_L.max(max_line);
                    let _ = pwriteln!(w, "{}", fmt(l, w, c, m, max_line, fname));
                }
                Err(e) => {
                    eprintln!("wc: stdin: {}", e);
                    exit_code = 1;
                }
            }
            continue;
        }
        let file = match File::open(fname) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("wc: {}: {}", fname, e);
                exit_code = 1;
                continue;
            }
        };
        let mut reader = BufReader::new(file);
        match count_stream(&mut reader) {
            Ok((l, w, c, m, max_line)) => {
                total_l += l;
                total_w += w;
                total_c += c;
                total_m += m;
                total_L = total_L.max(max_line);
                let _ = pwriteln!(w, "{}", fmt(l, w, c, m, max_line, fname));
            }
            Err(e) => {
                eprintln!("wc: {}: {}", fname, e);
                exit_code = 1;
            }
        }
    }

    if files.len() > 1 {
        let _ = pwriteln!(w, "{}", fmt(total_l, total_w, total_c, total_m, total_L, "total"));
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_data(data: &str) -> (usize, usize, usize, usize, usize) {
        let mut reader = data.as_bytes();
        count_stream(&mut reader).unwrap()
    }

    #[test]
    fn test_count_data_empty() {
        assert_eq!(count_data(""), (0, 0, 0, 0, 0));
    }

    #[test]
    fn test_count_data_basic() {
        let result = count_data("hello world\n");
        assert_eq!(result, (1, 2, 12, 12, 11));
    }

    #[test]
    fn test_count_data_multiline() {
        let result = count_data("a\nbb\nccc\n");
        assert_eq!(result, (3, 3, 8, 8, 3));
    }

    #[test]
    fn test_count_data_no_trailing_newline() {
        let result = count_data("hello world");
        assert_eq!(result, (0, 2, 11, 11, 11));
    }

    #[test]
    fn test_count_data_mixed_whitespace() {
        // tabs and spaces as word separators
        let result = count_data("a\tb  c\n");
        assert_eq!(result, (1, 3, 8, 8, 6));
    }

    #[test]
    fn test_count_data_utf8() {
        // "héllo wörld\n" is 13 bytes but 12 chars
        let (l, w, c, m, max_line) = count_data("héllo wörld\n");
        assert_eq!(l, 1);
        assert_eq!(w, 2);
        assert_eq!(c, 13);
        assert_eq!(m, 12);
        assert_eq!(max_line, 13);
    }

    #[test]
    fn test_count_data_max_line() {
        let (l, w, c, m, max_line) = count_data("short\nlonger_line\nshort\n");
        assert_eq!(l, 3);
        assert_eq!(w, 3);
        assert_eq!(c, 25);
        assert_eq!(m, 21);
        assert_eq!(max_line, 12);
    }

    #[test]
    fn test_count_data_max_line_no_trailing_newline() {
        let (l, w, c, m, max_line) = count_data("short\nlongest");
        assert_eq!(l, 1);
        assert_eq!(w, 2);
        assert_eq!(max_line, 7, "should be length of 'longest' (7), not 'short' (5)");
    }

    #[test]
    fn test_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into(), "/dev/null".into()]), 1);
    }

    #[test]
    fn test_valid_options() {
        assert_eq!(run(&mut std::io::sink(), &["-l".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-w".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-c".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-m".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-L".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_combined_options() {
        assert_eq!(run(&mut std::io::sink(), &["-lw".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-lwm".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-lwcm".into(), "/dev/null".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["-L".into(), "/dev/null".into()]), 0);
    }
}
