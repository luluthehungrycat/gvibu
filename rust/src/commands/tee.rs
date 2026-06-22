use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut append = false;
    let mut files: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-a" {
            append = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("tee: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let mut outputs: Vec<Box<dyn Write>> = Vec::new();

    // Open output files
    let mut exit_code = 0;
    for fname in &files {
        let file = if append {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(fname)
        } else {
            File::create(fname)
        };

        match file {
            Ok(f) => outputs.push(Box::new(f)),
            Err(e) => {
                eprintln!("tee: {}: {}", fname, e);
                exit_code = 1;
            }
        }
    }

    // Read stdin and write to all outputs
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let data = format!("{}\n", l);
                for out in outputs.iter_mut() {
                    if let Err(e) = out.write_all(data.as_bytes()) {
                        eprintln!("tee: write error: {}", e);
                        exit_code = 1;
                    }
                }
                if let Err(_e) = writeln!(w, "{}", l) {
                    return 1;
                }
            }
            Err(e) => {
                eprintln!("tee: read error: {}", e);
                return 1;
            }
        }
    }

    // Flush all outputs
    for out in outputs.iter_mut() {
        let _ = out.flush();
    }
    let _ = w.flush();

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tee_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_tee_append_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-a".into()]), 0);
    }
}
