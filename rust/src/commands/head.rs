/// head: output the first part of files.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn print_lines(lines: &[String], num_lines: usize) {
    let end = num_lines.min(lines.len());
    for line in &lines[..end] {
        let _ = write!(io::stdout(), "{}", line);
    }
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

pub fn run(args: &[String]) -> i32 {
    let mut num_lines: usize = 10;
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
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
        let stdin = io::stdin();
        let lines = read_n_lines(stdin.lock(), num_lines);
        print_lines(&lines, num_lines);
        return 0;
    }

    for (idx, fname) in files.iter().enumerate() {
        if files.len() > 1 {
            if idx > 0 {
                let _ = write!(io::stdout(), "\n");
            }
            let _ = writeln!(io::stdout(), "==> {} <==", fname);
        }

        if fname == "-" {
            let stdin = io::stdin();
            let lines = read_n_lines(stdin.lock(), num_lines);
            print_lines(&lines, num_lines);
        } else {
            match File::open(fname) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let lines = read_n_lines(reader, num_lines);
                    print_lines(&lines, num_lines);
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
