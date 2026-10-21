/// rm: remove files or directories.
use std::io::Write;
use std::fs;
use std::path::Path;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut recursive = false;
    let mut force = false;
    let mut verbose = false;
    let mut dir_only = false;
    let mut interactive = false;
    let mut preserve_root = true;
    let mut targets: Vec<String> = Vec::new();

    let mut i = 0;
    let mut end_options = false;
    while i < args.len() {
        let arg = &args[i];
        if end_options {
            targets.push(arg.clone());
            i += 1;
            continue;
        }
        if arg == "--" {
            end_options = true;
            i += 1;
            continue;
        }
        if arg == "-r" || arg == "-R" || arg == "--recursive" {
            recursive = true;
        } else if arg == "-f" || arg == "--force" {
            force = true;
        } else if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg == "-d" || arg == "--dir" {
            dir_only = true;
        } else if arg == "-i" || arg == "--interactive" {
            interactive = true;
        } else if arg == "--preserve-root" {
            preserve_root = true;
        } else if arg == "--no-preserve-root" {
            preserve_root = false;
        } else if arg == "-rf" || arg == "-fr" || arg == "-Rf" || arg == "-fR" {
            recursive = true;
            force = true;
        } else if arg == "-dv" || arg == "-vd" {
            dir_only = true;
            verbose = true;
        } else if arg == "-iv" || arg == "-vi" {
            interactive = true;
            verbose = true;
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

    // Check for root removal with preserve_root after resolving path aliases.
    if preserve_root {
        for target in &targets {
            let resolved = fs::canonicalize(target).unwrap_or_else(|_| Path::new(target).to_path_buf());
            if resolved == Path::new("/") {
                eprintln!("rm: it is dangerous to operate recursively on '/'");
                eprintln!("rm: use --no-preserve-root to override this failsafe");
                return 1;
            }
        }
    }

    let mut exit_code = 0;

    for target in &targets {
        let path = Path::new(target);

        // Handle interactive mode
        if interactive && !force {
            let prompt = format!("rm: remove '{}'? ", target);
            eprint!("{}", prompt);
            let _ = std::io::stdout().flush();

            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                eprintln!("rm: cannot read input: {}", e);
                return 1;
            }

            let input = input.trim();
            if input != "y" && input != "Y" && input != "yes" && input != "Yes" {
                continue; // Skip this file
            }
        }

        let result = if dir_only {
            match fs::symlink_metadata(path) {
                Ok(metadata) if metadata.is_dir() => fs::remove_dir(target),
                Ok(_) => fs::remove_file(target),
                Err(error) => Err(error),
            }
        } else if recursive {
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

    #[test]
    fn test_rm_preserve_root() {
        assert_eq!(run(&mut std::io::sink(), &["/".into()]), 1);
    }

    #[test]
    fn test_rm_preserve_root_alias() {
        assert_eq!(run(&mut std::io::sink(), &["-r".into(), "/tmp/..".into()]), 1);
    }

    #[test]
    fn test_rm_no_preserve_root() {
        // This should not fail due to preserve_root, but will fail because we can't actually remove /
        // We're mainly testing that --no-preserve-root doesn't trigger the preserve_root error
        assert_eq!(run(&mut std::io::sink(), &["--no-preserve-root".into(), "/".into()]), 1);
    }

    #[test]
    fn dir_flag_removes_files_and_empty_directories_only() {
        let root = std::env::temp_dir().join(format!("gvibu-rm-dir-{}", std::process::id()));
        let empty = root.join("empty");
        let nonempty = root.join("nonempty");
        std::fs::create_dir_all(&empty).unwrap();
        std::fs::create_dir_all(&nonempty).unwrap();
        std::fs::write(nonempty.join("child"), b"x").unwrap();
        let file = root.join("file");
        std::fs::write(&file, b"x").unwrap();

        let file_result = run(&mut std::io::sink(), &["-d".into(), file.to_string_lossy().into_owned()]);
        let empty_result = run(&mut std::io::sink(), &["-d".into(), empty.to_string_lossy().into_owned()]);
        let nonempty_result = run(&mut std::io::sink(), &["-d".into(), nonempty.to_string_lossy().into_owned()]);

        assert_eq!(file_result, 0);
        assert_eq!(empty_result, 0);
        assert_eq!(nonempty_result, 1);
        assert!(!file.exists());
        assert!(!empty.exists());
        assert!(nonempty.join("child").exists());
        std::fs::remove_dir_all(root).unwrap();
    }
}
