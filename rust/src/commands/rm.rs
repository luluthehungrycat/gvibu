/// rm: remove files or directories.
use std::io::Write;
use std::fs;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut recursive = false;
    let mut force = false;
    let mut verbose = false;
    let mut targets: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-r" || arg == "-R" || arg == "--recursive" {
            recursive = true;
        } else if arg == "-f" || arg == "--force" {
            force = true;
        } else if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg == "-rf" || arg == "-fr" || arg == "-Rf" || arg == "-fR" {
            recursive = true;
            force = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("rm: invalid option: {}", arg);
            return 1;
        } else {
            targets.push(arg.clone());
        }
        i += 1;
    }

    if targets.is_empty() {
        eprintln!("rm: missing operand");
        return 1;
    }

    let mut exit_code = 0;

    for target in &targets {
        let result = if recursive {
            fs::remove_dir_all(target)
        } else {
            fs::remove_file(target)
        };

        match result {
            Ok(_) => {
                if verbose {
                    let _ = writeln!(stdout, "removed '{}'", target);
                }
            }
            Err(e) => {
                if !force {
                    eprintln!("rm: cannot remove '{}': {}", target, e);
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
    fn test_rm_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_rm_nonexistent() {
        assert_eq!(run(&mut std::io::sink(), &["/nonexistent_file_xyz".into()]), 1);
    }

    #[test]
    fn test_rm_force_nonexistent() {
        assert_eq!(run(&mut std::io::sink(), &["-f".into(), "/nonexistent_file_xyz".into()]), 0);
    }

    #[test]
    fn test_rm_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }
}
