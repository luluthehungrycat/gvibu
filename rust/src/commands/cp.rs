/// cp: copy files and directories.
use std::fs;
use std::io::Write;
use crate::pwriteln;

fn basename(path: &str) -> String {
    let s = path.trim_end_matches('/');
    match s.rfind('/') {
        Some(i) => s[i + 1..].to_string(),
        None => s.to_string(),
    }
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut interactive = false;
    let mut force = false;
    let mut verbose = false;
    let mut no_clobber = false;
    let mut update = false;
    let mut recursive = false;
    let mut sources: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "-i" | "--interactive" => interactive = true,
            "-f" | "--force" => force = true,
            "-v" | "--verbose" => verbose = true,
            "-n" | "--no-clobber" => no_clobber = true,
            "-u" | "--update" => update = true,
            "-r" | "-R" | "--recursive" => recursive = true,
            "--" => { break; }
            s if s.starts_with('-') && s.len() > 1 => {
                eprintln!("cp: invalid option: {}", s);
                return 1;
            }
            s => sources.push(s),
        }
        i += 1;
    }

    if sources.len() < 2 {
        eprintln!("cp: missing operand");
        return 1;
    }

    // safe: sources.len() >= 2 verified above
    let dest = &sources[sources.len() - 1];
    let srcs = &sources[..sources.len() - 1];

    // Determine if dest is an existing directory
    let dest_is_dir = fs::metadata(dest).map(|m| m.is_dir()).unwrap_or(false);

    // If multiple sources, dest must be an existing directory
    if srcs.len() > 1 && !dest_is_dir {
        eprintln!("cp: target '{}' is not a directory", dest);
        return 1;
    }

    if interactive && force {
        force = false; // -i overrides -f
    }

    let mut exit_code = 0;
    for src in srcs {
        let dest_path = if dest_is_dir {
            format!("{}/{}", dest, basename(src))
        } else {
            dest.to_string()
        };

        exit_code |= copy_one(
            w, src, &dest_path,
            interactive, force, verbose, no_clobber, update, recursive,
        );
    }

    exit_code
}

fn copy_one(
    w: &mut dyn Write,
    src: &str,
    dst: &str,
    interactive: bool,
    force: bool,
    verbose: bool,
    no_clobber: bool,
    recursive: bool,
    update: bool,
) -> i32 {
    let src_meta = match fs::symlink_metadata(src) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("cp: cannot stat '{}': {}", src, e);
            return 1;
        }
    };

    if src_meta.is_dir() {
        if !recursive {
            eprintln!("cp: -r not specified; omitting directory '{}'", src);
            return 1;
        }
        return copy_dir(w, src, dst, interactive, force, verbose, no_clobber, update);
    }

    // Update check: only copy if source is newer than destination
    if update {
        if let Ok(dst_meta) = fs::metadata(dst) {
            if let (Ok(src_mtime), Ok(dst_mtime)) = (src_meta.modified(), dst_meta.modified()) {
                if src_mtime <= dst_mtime {
                    if verbose {
                        pwriteln!(w, "skip '{}' (newer or equal)", dst);
                    }
                    return 0;
                }
            }
        }
    }

    // No-clobber check
    if no_clobber && fs::metadata(dst).is_ok() {
        if verbose {
            pwriteln!(w, "skip '{}' (no-clobber)", dst);
        }
        return 0;
    }

    // Interactive check
    if interactive && fs::metadata(dst).is_ok() {
        eprint!("cp: overwrite '{}'? ", dst);
        let _ = std::io::Write::flush(&mut std::io::stderr());
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                return 0;
            }
        }
    }

    // Force: if dest exists and is writable, remove it first
    if force {
        if let Ok(dst_meta) = fs::metadata(dst) {
            if dst_meta.is_file() {
                let _ = fs::remove_file(dst);
            } else if dst_meta.is_dir() {
                let _ = fs::remove_dir_all(dst);
            }
        }
    }

    match fs::copy(src, dst) {
        Ok(_) => {
            if verbose {
                pwriteln!(w, "'{}' -> '{}'", src, dst);
            }
            0
        }
        Err(e) => {
            eprintln!("cp: cannot copy '{}' to '{}': {}", src, dst, e);
            1
        }
    }
}

fn copy_dir(
    w: &mut dyn Write,
    src: &str,
    dst: &str,
    interactive: bool,
    force: bool,
    verbose: bool,
    no_clobber: bool,
    update: bool,
) -> i32 {
    // Create destination directory
    if let Err(e) = fs::create_dir_all(dst) {
        eprintln!("cp: cannot create directory '{}': {}", dst, e);
        return 1;
    }

    let dir = match fs::read_dir(src) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cp: cannot read directory '{}': {}", src, e);
            return 1;
        }
    };

    let mut exit_code = 0;
    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("cp: error reading '{}': {}", src, e);
                exit_code = 1;
                continue;
            }
        };

        let src_path = entry.path();
        let name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Skip hidden files? No, copy everything
        let dst_path = format!("{}/{}", dst, name);
        let src_str = src_path.to_string_lossy();

        let ft = match entry.file_type() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("cp: cannot stat '{}': {}", src_str, e);
                exit_code = 1;
                continue;
            }
        };

        if ft.is_dir() {
            exit_code |= copy_dir(
                w, &src_str, &dst_path,
                interactive, force, verbose, no_clobber, update,
            );
        } else {
            exit_code |= copy_one(
                w, &src_str, &dst_path,
                interactive, force, verbose, no_clobber, update, true,
            );
        }
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cp_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_cp_one_arg() {
        assert_eq!(run(&mut std::io::sink(), &["a".into()]), 1);
    }

    #[test]
    fn test_cp_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_cp_nonexistent_src() {
        assert_eq!(run(&mut std::io::sink(), &["/nonexistent_cp_src_xyz".into(), "/tmp/cp_test_dest".into()]), 1);
    }

    #[test]
    fn test_cp_to_self() {
        // Use std::io::sink to avoid actually creating files
        // This tests the argument parsing at least
        assert_eq!(run(&mut std::io::sink(), &["-v".into(), "a".into(), "b".into()]), 1);
    }

    #[test]
    fn test_cp_dev_null_to_file() {
        let tmp_src = std::env::temp_dir().join("gvibu_cp_src_test");
        let tmp_dst = std::env::temp_dir().join("gvibu_cp_dst_test");
        let _ = std::fs::write(&tmp_src, "test content");
        let src_str = tmp_src.to_str().unwrap().to_string();
        let dst_str = tmp_dst.to_str().unwrap().to_string();
        let result = run(&mut std::io::sink(), &[src_str.into(), dst_str.into()]);
        let _ = std::fs::remove_file(&tmp_src);
        let _ = std::fs::remove_file(&tmp_dst);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_basename() {
        assert_eq!(basename("/foo/bar"), "bar");
        assert_eq!(basename("bar"), "bar");
        assert_eq!(basename("/foo/bar/"), "bar");
        assert_eq!(basename("/"), "");
    }
}
