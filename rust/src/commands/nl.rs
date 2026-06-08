/// nl: number lines of files.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut files: Vec<String> = Vec::new();
    let mut body_start = 1usize;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg == "-v" {
            i += 1;
            if i >= args.len() {
                eprintln!("nl: option requires an argument: -v");
                return 1;
            }
            match args[i].parse::<usize>() {
                Ok(n) => body_start = n,
                Err(_) => {
                    eprintln!("nl: invalid starting line number: {}", args[i]);
                    return 1;
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("nl: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let mut line_num = body_start;
    let mut exit_code = 0;

    if files.is_empty() {
        let stdin = io::stdin();
        exit_code = number_lines(w, stdin.lock(), &mut line_num);
    } else {
        for fname in files {
            if fname == "-" {
                let stdin = io::stdin();
                exit_code |= number_lines(w, stdin.lock(), &mut line_num);
            } else {
                match File::open(&fname) {
                    Ok(file) => {
                        exit_code |= number_lines(w, BufReader::new(file), &mut line_num);
                    }
                    Err(e) => {
                        eprintln!("nl: {}: {}", fname, e);
                        exit_code = 1;
                    }
                }
            }
        }
    }

    exit_code
}

fn number_lines<R: BufRead>(w: &mut dyn Write, reader: R, line_num: &mut usize) -> i32 {
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    // Empty lines: don't number, just output
                    pwriteln!(w);
                } else {
                    pwriteln!(w, "{:>6}\t{}", line_num, l);
                    *line_num += 1;
                }
            }
            Err(e) => {
                eprintln!("nl: read error: {}", e);
                return 1;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::sink;

    #[test]
    fn test_nl_dev_null() {
        assert_eq!(run(&mut sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_nl_no_args() {
        assert_eq!(run(&mut sink(), &[]), 0);
    }

    #[test]
    fn test_nl_invalid_option() {
        assert_eq!(run(&mut sink(), &["-x".into()]), 1);
    }
}
