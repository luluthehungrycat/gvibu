/// tac: concatenate and write files in reverse.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut files: Vec<String> = Vec::new();
    let mut exit_code = 0;

    for arg in args {
        if arg == "--" { break; }
        if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("tac: invalid option: {}", arg);
            return 1;
        }
        files.push(arg.clone());
    }

    if files.is_empty() {
        let stdin = io::stdin();
        let lines = read_all_lines(stdin.lock());
        for line in lines.iter().rev() {
            pwriteln!(w, "{}", line);
        }
        return 0;
    }

    for fname in &files {
        if fname == "-" {
            let stdin = io::stdin();
            let lines = read_all_lines(stdin.lock());
            for line in lines.iter().rev() {
                pwriteln!(w, "{}", line);
            }
        } else {
            match File::open(fname) {
                Ok(file) => {
                    let lines = read_all_lines(BufReader::new(file));
                    for line in lines.iter().rev() {
                        pwriteln!(w, "{}", line);
                    }
                }
                Err(e) => {
                    eprintln!("tac: {}: {}", fname, e);
                    exit_code = 1;
                }
            }
        }
    }

    exit_code
}

fn read_all_lines<R: BufRead>(reader: R) -> Vec<String> {
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(_) => break,
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tac_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_tac_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_read_all_lines_empty() {
        let lines = read_all_lines("".as_bytes());
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_all_lines_basic() {
        let lines = read_all_lines("a\nb\nc\n".as_bytes());
        assert_eq!(lines, vec!["a", "b", "c"]);
    }
}
