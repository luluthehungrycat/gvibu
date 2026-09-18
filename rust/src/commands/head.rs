/// head: output the first part of files.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use crate::pwriteln;

/// Print line-based output (existing -n behavior).
fn print_lines(w: &mut dyn Write, lines: &[String], num_lines: usize) -> i32 {
    let end = num_lines.min(lines.len());
    for line in &lines[..end] {
        if let Err(e) = write!(w, "{}", line) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    }
    0
}

fn read_n_lines<R: BufRead>(reader: R, num_lines: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                lines.push(l + "\n");
                if lines.len() >= num_lines {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    lines
}

/// Print byte-based output (new -c behavior).
fn print_bytes(w: &mut dyn Write, data: &[u8], num_bytes: usize) -> i32 {
    let end = num_bytes.min(data.len());
    if let Err(e) = w.write_all(&data[..end]) {
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            return 0;
        }
        return 1;
    }
    0
}

fn read_n_bytes<R: Read>(mut reader: R, num_bytes: usize) -> Vec<u8> {
    let mut buf = vec![0u8; num_bytes];
    let mut total = 0;
    loop {
        match reader.read(&mut buf[total..]) {
            Ok(0) => break,
            Ok(n) => {
                total += n;
                if total >= num_bytes {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    buf.truncate(total);
    buf
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut num_lines: usize = 10;
    let mut num_bytes: Option<usize> = None;
    let mut quiet = false;
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-n" {
            i += 1;
            if i >= args.len() {
                eprintln!("head: option requires an argument: -n");
                return 1;
            }
            match args[i].parse::<i32>() {
                Ok(n) if n >= 0 => num_lines = n as usize,
                Ok(_) => {
                    eprintln!("head: invalid number of lines: 0");
                    return 1;
                }
                Err(_) => {
                    eprintln!("head: invalid number of lines: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-q" {
            quiet = true;
        } else if arg == "-c" {
            i += 1;
            if i >= args.len() {
                eprintln!("head: option requires an argument: -c");
                return 1;
            }
            match args[i].parse::<i64>() {
                Ok(n) if n >= 0 => num_bytes = Some(n as usize),
                Ok(_) => {
                    eprintln!("head: invalid number of bytes: 0");
                    return 1;
                }
                Err(_) => {
                    eprintln!("head: invalid number of bytes: {}", args[i]);
                    return 1;
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("head: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let mut exit_code = 0;

    if files.is_empty() {
        if let Some(nbytes) = num_bytes {
            let stdin = io::stdin();
            let data = read_n_bytes(stdin.lock(), nbytes);
            return print_bytes(w, &data, nbytes);
        } else {
            let stdin = io::stdin();
            let lines = read_n_lines(stdin.lock(), num_lines);
            return print_lines(w, &lines, num_lines);
        }
    }

    for (idx, fname) in files.iter().enumerate() {
        if files.len() > 1 && !quiet {
            if idx > 0 {
                pwriteln!(w);
            }
            pwriteln!(w, "==> {} <==", fname);
        }

        if fname == "-" {
            if let Some(nbytes) = num_bytes {
                let stdin = io::stdin();
                let data = read_n_bytes(stdin.lock(), nbytes);
                exit_code |= print_bytes(w, &data, nbytes);
            } else {
                let stdin = io::stdin();
                let lines = read_n_lines(stdin.lock(), num_lines);
                exit_code |= print_lines(w, &lines, num_lines);
            }
        } else {
            match File::open(fname) {
                Ok(file) => {
                    if let Some(nbytes) = num_bytes {
                        let data = read_n_bytes(file, nbytes);
                        exit_code |= print_bytes(w, &data, nbytes);
                    } else {
                        let reader = BufReader::new(file);
                        let lines = read_n_lines(reader, num_lines);
                        exit_code |= print_lines(w, &lines, num_lines);
                    }
                }
                Err(e) => {
                    eprintln!("head: {}: {}", fname, e);
                    exit_code = 1;
                }
            }
        }
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_head_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_head_c_flag_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["-c".into(), "5".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_head_c_missing_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-c".into()]), 1);
    }

    #[test]
    fn test_head_c_zero() {
        assert_eq!(run(&mut std::io::sink(), &["-c".into(), "0".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_read_n_bytes() {
        let data = read_n_bytes("hello world".as_bytes(), 5);
        assert_eq!(data, b"hello");
    }

    #[test]
    fn test_read_n_bytes_less_than_n() {
        let data = read_n_bytes("hi".as_bytes(), 10);
        assert_eq!(data, b"hi");
    }
}
