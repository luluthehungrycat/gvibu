/// mv: move (rename) files.
use std::io::Write;
use std::fs;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut interactive = false;
    let mut force = false;
    let mut verbose = false;
    let mut no_clobber = false;
    let mut targets: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-i" {
            interactive = true;
        } else if arg == "-f" {
            force = true;
        } else if arg == "-v" {
            verbose = true;
        } else if arg == "-n" {
            no_clobber = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("mv: invalid option: {}", arg);
            return 1;
        } else {
            targets.push(arg.clone());
        }
        i += 1;
    }

    if targets.len() < 2 {
        eprintln!("mv: missing operand");
        return 1;
    }


    let src = &targets[0];
    let dst = &targets[1];

    if no_clobber && fs::metadata(dst).is_ok() {
        return 0;
    }

    if interactive && !force && fs::metadata(dst).is_ok() {
        eprint!("mv: overwrite '{}'? ", dst);
        let _ = std::io::Write::flush(&mut std::io::stderr());
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                return 0;
            }
        }
    }

    match fs::rename(src, dst) {
        Ok(_) => {
            if verbose {
                let _ = writeln!(stdout, "renamed '{}' -> '{}'", src, dst);
            }
            0
        }
        Err(e) => {
            eprintln!("mv: cannot move '{}' to '{}': {}", src, dst, e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mv_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_mv_one_arg() {
        assert_eq!(run(&mut std::io::sink(), &["a".into()]), 1);
    }

    #[test]
    fn test_mv_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }
}
