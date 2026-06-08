/// printf: format and print data.
use std::io::Write;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("printf: missing operand");
        return 1;
    }

    let format_str = interpret_escapes(&args[0]);
    let mut format_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    if format_args.is_empty() {
        format_args.push(""); // at least one empty arg for recycling
    }

    let format_bytes = format_str.as_bytes();
    let len = format_bytes.len();
    let mut i = 0;
    let mut arg_idx = 0;
    let mut exit_code = 0;

    while i < len {
        if format_bytes[i] == b'\\' {
            // Handle escape sequences in format string
            if i + 1 < len {
                let next = format_bytes[i + 1];
                let ch = match next {
                    b'n' => '\n',
                    b't' => '\t',
                    b'r' => '\r',
                    b'\\' => '\\',
                    b'"' => '"',
                    b'0' => {
                        // Octal escape: \0NNN
                        let mut oct_str = String::new();
                        let mut j = i + 2;
                        while j < len && j < i + 5 && format_bytes[j] >= b'0' && format_bytes[j] <= b'7' {
                            oct_str.push(format_bytes[j] as char);
                            j += 1;
                        }
                        if let Ok(val) = u32::from_str_radix(&oct_str, 8) {
                            if let Some(ch) = char::from_u32(val) {
                                ch
                            } else {
                                ' '
                            }
                        } else {
                            ' '
                        }
                    }
                    b' ' | b'!'..=b'\'' | b')'..=b'/' | b':'..=b'@' | b'A'..=b'`' | b'{'..=b'~' => next as char,
                    _ => next as char,
                };
                pwrite!(w, "{}", ch);
                if next == b'0' {
                    let mut skip = 2;
                    while i + skip < len && skip < 5 && format_bytes[i + skip] >= b'0' && format_bytes[i + skip] <= b'7' {
                        skip += 1;
                    }
                    i += skip - 1;
                } else {
                    i += 2;
                }
                continue;
            }
            i += 1;
            continue;
        }

        if format_bytes[i] == b'%' {
            i += 1;
            // Parse optional width
            let mut width_str = String::new();
            while i < len && format_bytes[i].is_ascii_digit() {
                width_str.push(format_bytes[i] as char);
                i += 1;
            }
            let width: usize = width_str.parse().unwrap_or(0);

            if i >= len {
                break;
            }

            let spec = format_bytes[i];
            let arg = if arg_idx < format_args.len() {
                format_args[arg_idx]
            } else {
                // Recycle from start
                format_args[arg_idx % format_args.len()]
            };
            arg_idx += 1;

            match spec {
                b'%' => {
                    pwrite!(w, "%");
                }
                b's' => {
                    if width > 0 && arg.len() < width {
                        print_padded(w, arg, width);
                    } else {
                        pwrite!(w, "{}", arg);
                    }
                }
                b'd' | b'i' => {
                    let val: i64 = arg.parse().unwrap_or(0);
                    if width > 0 {
                        pwrite!(w, "{:>width$}", val, width = width);
                    } else {
                        pwrite!(w, "{}", val);
                    }
                }
                b'u' => {
                    let val: u64 = arg.parse().unwrap_or(0);
                    if width > 0 {
                        pwrite!(w, "{:>width$}", val, width = width);
                    } else {
                        pwrite!(w, "{}", val);
                    }
                }
                b'o' => {
                    let val: i64 = arg.parse().unwrap_or(0);
                    if width > 0 {
                        pwrite!(w, "{:>width$o}", val, width = width);
                    } else {
                        pwrite!(w, "{:o}", val);
                    }
                }
                b'x' => {
                    let val: i64 = arg.parse().unwrap_or(0);
                    if width > 0 {
                        pwrite!(w, "{:>width$x}", val, width = width);
                    } else {
                        pwrite!(w, "{:x}", val);
                    }
                }
                b'X' => {
                    let val: i64 = arg.parse().unwrap_or(0);
                    if width > 0 {
                        pwrite!(w, "{:>width$X}", val, width = width);
                    } else {
                        pwrite!(w, "{:X}", val);
                    }
                }
                b'c' => {
                    let ch = arg.chars().next().unwrap_or(' ');
                    if width > 1 {
                        pwrite!(w, "{:>width$}", ch.to_string(), width = width);
                    } else {
                        pwrite!(w, "{}", ch);
                    }
                }
                b'f' | b'e' | b'E' | b'g' | b'G' => {
                    // Float - parse as f64
                    let val: f64 = arg.parse().unwrap_or(0.0);
                    if width > 0 {
                        pwrite!(w, "{:>width$}", val, width = width);
                    } else {
                        pwrite!(w, "{}", val);
                    }
                }
                _ => {
                    // Unknown format specifier - print literal
                    pwrite!(w, "%{}", spec as char);
                }
            }
            i += 1;
            continue;
        }

        // Regular character
        pwrite!(w, "{}", format_bytes[i] as char);
        i += 1;
    }

    // Format string doesn't add trailing newline by default
    exit_code
}

fn print_padded(w: &mut dyn Write, s: &str, width: usize) {
    pwrite!(w, "{:>width$}", s, width = width);
}

fn interpret_escapes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'n' => result.push('\n'),
                b't' => result.push('\t'),
                b'r' => result.push('\r'),
                b'\\' => result.push('\\'),
                b'"' => result.push('"'),
                b'0' => {
                    let mut oct_str = String::new();
                    let mut j = i + 2;
                    while j < bytes.len() && j < i + 5 && bytes[j] >= b'0' && bytes[j] <= b'7' {
                        oct_str.push(bytes[j] as char);
                        j += 1;
                    }
                    if let Ok(val) = u32::from_str_radix(&oct_str, 8) {
                        if let Some(ch) = char::from_u32(val) {
                            result.push(ch);
                        }
                    }
                    i = j - 1;
                    i += 1;
                    continue;
                }
                c => result.push(c as char),
            }
            i += 2;
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printf_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_printf_no_format_args() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["hello\n".into()]);
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "hello\n");
    }

    #[test]
    fn test_printf_s_specifier() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["%s\n".into(), "hello".into()]);
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "hello\n");
    }

    #[test]
    fn test_printf_d_specifier() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["%d\n".into(), "42".into()]);
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "42\n");
    }

    #[test]
    fn test_printf_x_specifier() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["%x\n".into(), "255".into()]);
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "ff\n");
    }

    #[test]
    fn test_printf_percent() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["%%\n".into()]);
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "%\n");
    }

    #[test]
    fn test_printf_escapes() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["a\\tb".into()]);
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "a\tb");
    }

    #[test]
    fn test_printf_octal() {
        let mut buf = Vec::new();
        let ret = run(&mut buf, &["\\0101".into()]);  // octal 101 = 65 = 'A'
        assert_eq!(ret, 0);
        assert_eq!(String::from_utf8_lossy(&buf), "A");
    }

    #[test]
    fn test_interpret_escapes() {
        assert_eq!(interpret_escapes("a\\tb"), "a\tb");
        assert_eq!(interpret_escapes("a\\nb"), "a\nb");
        assert_eq!(interpret_escapes("a\\\\b"), "a\\b");
        assert_eq!(interpret_escapes("\\0101"), "A");
        assert_eq!(interpret_escapes("hello"), "hello");
    }
}
