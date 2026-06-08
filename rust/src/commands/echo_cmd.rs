/// echo: print its arguments joined by spaces.
/// Supports -n (no newline), -e (enable escapes), -E (disable escapes, default).
use std::io::Write;

fn interpret_escapes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some('\'') => out.push('\''),
                Some('\"') => out.push('\"'),
                Some('0') => {
                    // Parse up to 3 octal digits
                    let mut octal = String::with_capacity(3);
                    for _ in 0..3 {
                        match chars.next() {
                            Some(d) if d.is_ascii_digit() && d <= '7' => octal.push(d),
                            Some(d) => {
                                // Push back: put the char back via a temp string
                                // We'll handle this by pushing d onto output
                                out.push(d);
                                break;
                            }
                            None => break,
                        }
                    }
                    if let Ok(val) = u32::from_str_radix(&octal, 8) {
                        if let Some(ch) = char::from_u32(val) {
                            out.push(ch);
                        }
                    }
                }
                Some(c) => {
                    out.push('\\');
                    out.push(c);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut newline = true;
    let mut enable_escapes = false;
    let mut start_idx = 0;

    while start_idx < args.len() {
        let arg = &args[start_idx];
        if arg == "--" { start_idx += 1; break; }
        if arg == "-n" {
            newline = false;
            start_idx += 1;
        } else if arg == "-e" {
            enable_escapes = true;
            start_idx += 1;
        } else if arg == "-E" {
            enable_escapes = false;
            start_idx += 1;
        } else {
            break;
        }
    }

    for (i, arg) in args.iter().enumerate().skip(start_idx) {
        if i > start_idx {
            pwrite!(w, " ");
        }
        if enable_escapes {
            pwrite!(w, "{}", interpret_escapes(arg));
        } else {
            pwrite!(w, "{}", arg);
        }
    }

    if newline {
        pwriteln!(w);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }

    #[test]
    fn test_echo_hello() {
        assert_eq!(run(&mut std::io::sink(), &["hello".into()]), 0);
    }

    #[test]
    fn test_echo_multiple_args() {
        assert_eq!(run(&mut std::io::sink(), &["hello".into(), "world".into()]), 0);
    }

    #[test]
    fn test_echo_n_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-n".into(), "hello".into()]), 0);
    }

    #[test]
    fn test_echo_e_flag_newline() {
        assert_eq!(run(&mut std::io::sink(), &["-e".into(), "hello\\nworld".into()]), 0);
    }

    #[test]
    fn test_echo_e_flag_tab() {
        assert_eq!(run(&mut std::io::sink(), &["-e".into(), "hello\\tworld".into()]), 0);
    }

    #[test]
    fn test_echo_e_flag_no_effect_without_escapes() {
        assert_eq!(run(&mut std::io::sink(), &["-e".into(), "hello".into()]), 0);
    }

    #[test]
    fn test_echo_e_with_n() {
        assert_eq!(run(&mut std::io::sink(), &["-n".into(), "-e".into(), "hello\\nworld".into()]), 0);
    }

    #[test]
    fn test_interpret_escapes_newline() {
        assert_eq!(interpret_escapes("hello\\nworld"), "hello\nworld");
    }

    #[test]
    fn test_interpret_escapes_tab() {
        assert_eq!(interpret_escapes("hello\\tworld"), "hello\tworld");
    }

    #[test]
    fn test_interpret_escapes_backslash() {
        assert_eq!(interpret_escapes("hello\\\\world"), "hello\\world");
    }

    #[test]
    fn test_interpret_escapes_no_escapes() {
        assert_eq!(interpret_escapes("hello world"), "hello world");
    }
}
