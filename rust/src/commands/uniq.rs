use std::io::{self, BufRead, Write};

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut show_unique = false;
    let mut show_repeated = false;
    let mut show_count = false;

    for arg in args {
        match arg.as_str() {
            "-u" => show_unique = true,
            "-d" => show_repeated = true,
            "-c" => show_count = true,
            _ => {
                eprintln!("uniq: invalid option -- '{}'", arg);
                return 1;
            }
        }
    }

    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => {
                eprintln!("uniq: read error: {}", e);
                return 1;
            }
        }
    }

    if lines.is_empty() {
        return 0;
    }

    // Count occurrences in each adjacent run
    let mut output_lines: Vec<(u64, String)> = Vec::new();
    let mut current = &lines[0];
    let mut count: u64 = 1;

    for line in &lines[1..] {
        if line == current {
            count += 1;
        } else {
            output_lines.push((count, current.clone()));
            current = line;
            count = 1;
        }
    }
    output_lines.push((count, current.clone()));

    for (c, line) in &output_lines {
        let print_it = if show_unique && !show_repeated {
            *c == 1
        } else if show_repeated && !show_unique {
            *c > 1
        } else {
            true
        };

        if print_it {
            if show_count {
                writeln!(stdout, "{:>4} {}", c, line).ok();
            } else {
                writeln!(stdout, "{}", line).ok();
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniq_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_uniq_u_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-u".into()]), 0);
    }

    #[test]
    fn test_uniq_d_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-d".into()]), 0);
    }

    #[test]
    fn test_uniq_c_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-c".into()]), 0);
    }
}
