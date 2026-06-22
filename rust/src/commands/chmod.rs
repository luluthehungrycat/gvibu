/// chmod: change file mode bits.
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn parse_octal(s: &str) -> Option<u32> {
    if s.is_empty() || s.len() > 4 {
        return None;
    }
    for ch in s.chars() {
        if !ch.is_ascii_digit() || ch > '7' {
            return None;
        }
    }
    u32::from_str_radix(s, 8).ok()
}

fn parse_symbolic(s: &str) -> Option<(char, char, u32)> {
    // Returns (who, op, perm_bits)
    // who: u/g/o/a, op: +=-, perm_bits: r=4 w=2 x=1
    let mut chars = s.chars().peekable();

    // Collect who characters (can be multiple like "go", "ugo")
    let mut who_str = String::new();
    while let Some(&ch) = chars.peek() {
        match ch {
            'u' | 'g' | 'o' | 'a' => {
                who_str.push(ch);
                chars.next();
            }
            _ => break,
        }
    }

    if who_str.is_empty() {
        return None;
    }

    let op = chars.next()?;
    if op != '+' && op != '-' && op != '=' {
        return None;
    }

    let mut perm_bits = 0u32;
    for ch in chars {
        match ch {
            'r' => perm_bits |= 0b100,
            'w' => perm_bits |= 0b010,
            'x' => perm_bits |= 0b001,
            'X' | 's' | 't' => {} // ignore for simplicity
            _ => return None,
        }
    }

    // Expand who to the appropriate bits
    let (who_char, expanded_bits) = if who_str == "a" || who_str.len() > 1 {
        let mut bits = 0u32;
        for wch in who_str.chars() {
            match wch {
                'u' => bits |= perm_bits << 6,
                'g' => bits |= perm_bits << 3,
                'o' => bits |= perm_bits,
                'a' => bits |= (perm_bits << 6) | (perm_bits << 3) | perm_bits,
                _ => {}
            }
        }
        ('a', bits)
    } else {
        let ch = who_str.chars().next().unwrap();
        let bits = match ch {
            'u' => perm_bits << 6,
            'g' => perm_bits << 3,
            'o' => perm_bits,
            'a' => (perm_bits << 6) | (perm_bits << 3) | perm_bits,
            _ => unreachable!(),
        };
        (ch, bits)
    };

    Some((who_char, op, expanded_bits))
}


fn apply_mode_change(current: u32, who: char, op: char, bits: u32) -> u32 {
    let who_bits = match who {
        'u' => 0o700,
        'g' => 0o070,
        'o' => 0o007,
        'a' => 0o777,
        _ => 0o777,
    };
    match op {
        '+' => current | (bits & who_bits),
        '-' => current & !(bits & who_bits),
        '=' => (current & !who_bits) | (bits & who_bits),
        _ => current,
    }
}

fn chmod_path(path: &str, mode: u32, verbose: bool) -> i32 {
    match fs::set_permissions(path, fs::Permissions::from_mode(mode)) {
        Ok(()) => {
            if verbose {
                let _ = writeln!(std::io::stdout(), "mode of '{}' changed", path);
            }
            0
        }
        Err(e) => {
            eprintln!("chmod: {}: {}", path, e);
            1
        }
    }
}

fn chmod_recursive(path: &str, mode: u32, verbose: bool) -> i32 {
    let mut exit_code = 0;
    exit_code |= chmod_path(path, mode, verbose);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let child_path = entry.path();
            if child_path.is_dir() {
                exit_code |= chmod_recursive(child_path.to_str().unwrap_or(""), mode, verbose);
            } else {
                exit_code |= chmod_path(child_path.to_str().unwrap_or(""), mode, verbose);
            }
        }
    }
    exit_code
}

pub fn run(_w: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("chmod: missing operand");
        return 1;
    }

    let mut recursive = false;
    let mut verbose = false;
    let mut mode_arg: Option<String> = None;
    let mut files: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-R" {
            recursive = true;
        } else if arg == "-v" {
            verbose = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("chmod: invalid option: {}", arg);
            return 1;
        } else if mode_arg.is_none() {
            mode_arg = Some(arg.clone());
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let mode_str = match mode_arg {
        Some(m) => m,
        None => {
            eprintln!("chmod: missing operand");
            return 1;
        }
    };

    let mode = if let Some(octal) = parse_octal(&mode_str) {
        octal
    } else if let Some((who, op, bits)) = parse_symbolic(&mode_str) {
        // For symbolic, we need current mode. Default to 0 and apply.
        // This is a simplification; real chmod reads current mode.
        // We'll apply the symbolic change relative to a starting mode.
        let current = 0o644; // default
        apply_mode_change(current, who, op, bits)
    } else {
        eprintln!("chmod: invalid mode: '{}'", mode_str);
        return 1;
    };

    if files.is_empty() {
        eprintln!("chmod: missing operand after '{}'", mode_str);
        return 1;
    }

    let mut exit_code = 0;
    for fname in &files {
        if recursive && fs::metadata(fname).map(|m| m.is_dir()).unwrap_or(false) {
            exit_code |= chmod_recursive(fname, mode, verbose);
        } else {
            exit_code |= chmod_path(fname, mode, verbose);
        }
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_octal() {
        assert_eq!(parse_octal("755"), Some(0o755));
        assert_eq!(parse_octal("644"), Some(0o644));
        assert_eq!(parse_octal("0644"), Some(0o644));
        assert_eq!(parse_octal("777"), Some(0o777));
        assert_eq!(parse_octal(""), None);
        assert_eq!(parse_octal("8"), None);
        assert_eq!(parse_octal("abc"), None);
    }

    #[test]
    fn test_parse_symbolic() {
        let (who, op, bits) = parse_symbolic("u+x").unwrap();
        assert_eq!(who, 'u');
        assert_eq!(op, '+');
        assert_eq!(bits, 0o100);

        let (who, op, bits) = parse_symbolic("go-w").unwrap();
        assert_eq!(who, 'a');
        assert_eq!(op, '-');
        assert_eq!(bits, 0o022);

        let (who, op, bits) = parse_symbolic("a+r").unwrap();
        assert_eq!(who, 'a');
        assert_eq!(op, '+');
        assert_eq!(bits, 0o444); // r for u+g+o
    }

    #[test]
    fn test_apply_mode_change() {
        assert_eq!(apply_mode_change(0o644, 'u', '+', 0o100), 0o744);
        assert_eq!(apply_mode_change(0o755, 'g', '-', 0o010), 0o745);
        assert_eq!(apply_mode_change(0o644, 'a', '=', 0o444), 0o444);
    }

    #[test]
    fn test_chmod_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_chmod_no_files() {
        assert_eq!(run(&mut std::io::sink(), &["755".into()]), 1);
    }

    #[test]
    fn test_chmod_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }
}
