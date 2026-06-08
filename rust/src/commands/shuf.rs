/// shuf: randomly permute lines from stdin or a file.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use rand::seq::SliceRandom;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let files: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let mut lines = Vec::new();

    if files.is_empty() || (files.len() == 1 && files[0] == "-") {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => lines.push(l),
                Err(e) => {
                    eprintln!("shuf: stdin: {}", e);
                    return 1;
                }
            }
        }
    } else if files.len() == 1 {
        match File::open(files[0]) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(l) => lines.push(l),
                        Err(e) => {
                            eprintln!("shuf: {}: {}", files[0], e);
                            return 1;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("shuf: {}: {}", files[0], e);
                return 1;
            }
        }
    } else {
        eprintln!("shuf: too many arguments");
        return 1;
    }

    let mut rng = rand::thread_rng();
    lines.shuffle(&mut rng);

    for line in &lines {
        pwriteln!(w, "{}", line);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuf_empty_stdin() {
        let mut out: Vec<u8> = Vec::new();
        // With empty stdin we can't test via sink, but we can test error cases
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_shuf_file_not_found() {
        let mut out: Vec<u8> = Vec::new();
        let code = run(&mut out, &["/nonexistent_shuf_file_xyz".into()]);
        assert_eq!(code, 1);
    }

    #[test]
    fn test_shuf_too_many_args() {
        let mut out: Vec<u8> = Vec::new();
        let code = run(&mut out, &["a".into(), "b".into()]);
        assert_eq!(code, 1);
    }
}
