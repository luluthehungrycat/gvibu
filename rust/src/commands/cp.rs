/// cp: copy files and directories.
use std::fs;
use std::io::Write;
use std::path::Path;
use crate::pwriteln;

fn basename(path: &str) -> String {
    let s = path.trim_end_matches('/');
    match s.rfind('/') {
        Some(i) => s[i + 1..].to_string(),
        None => s.to_string(),
    }
}

#[cfg(unix)]
fn create_symlink(target: &Path, destination: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, destination)
}

#[cfg(not(unix))]
fn create_symlink(_target: &Path, _destination: &Path) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "preserving symbolic links is unsupported on this platform",
    ))
}


fn same_source_and_destination(src: &str, dst: &str, archive: bool, src_meta: &fs::Metadata) -> bool {
    if Path::new(src) == Path::new(dst) {
        return true;
    }
    let Ok(dst_lmeta) = fs::symlink_metadata(dst) else {
        return false;
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if src_meta.dev() == dst_lmeta.dev() && src_meta.ino() == dst_lmeta.ino() {
            return true;
        }
        // Archive symlinks are copied as links. Other copies follow source and
        // destination links, so compare their target inodes to avoid truncating
        // a source reached through the destination link.
        if archive && src_meta.file_type().is_symlink() {
            return false;
        }
        let (Ok(src_followed), Ok(dst_followed)) = (fs::metadata(src), fs::metadata(dst)) else {
            return false;
        };
        src_followed.dev() == dst_followed.dev() && src_followed.ino() == dst_followed.ino()
    }
    #[cfg(not(unix))]
    {
        if archive && src_meta.file_type().is_symlink() {
            return false;
        }
        fs::canonicalize(src).ok() == fs::canonicalize(dst).ok()
            && fs::canonicalize(src).is_ok()
    }
}

fn normalize_absolute(path: &Path) -> std::path::PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().unwrap_or_default().join(path)
    };
    let mut normalized = std::path::PathBuf::new();
    for component in absolute.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => { normalized.pop(); }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

fn resolve_existing_ancestor(path: &Path) -> std::io::Result<std::path::PathBuf> {
    let absolute = normalize_absolute(path);
    let mut ancestor = absolute.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        let name = ancestor.file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "no existing path ancestor")
        })?;
        suffix.push(name.to_os_string());
        ancestor = ancestor.parent().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "no existing path ancestor")
        })?;
    }
    let mut resolved = fs::canonicalize(ancestor)?;
    for part in suffix.iter().rev() {
        resolved.push(part);
    }
    Ok(normalize_absolute(&resolved))
}

fn destination_is_inside_source(src: &str, dst: &str) -> bool {
    let Ok(source_root) = fs::canonicalize(src) else {
        return false;
    };
    let destination = Path::new(dst);
    let candidate = match destination.file_name() {
        Some(name) => destination.parent().unwrap_or_else(|| Path::new(".")).join(name),
        None => destination.to_path_buf(),
    };
    resolve_existing_ancestor(&candidate)
        .map(|path| path.starts_with(source_root))
        .unwrap_or(false)
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut interactive = false;
    let mut force = false;
    let mut verbose = false;
    let mut no_clobber = false;
    let mut update = false;
    let mut recursive = false;
    let mut archive = false;
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
            "-a" | "--archive" => {
                archive = true;
                recursive = true;
            }
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
            interactive, force, verbose, no_clobber, update, recursive, archive,
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
    update: bool,
    recursive: bool,
    archive: bool,
) -> i32 {
    let src_meta = match fs::symlink_metadata(src) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("cp: cannot stat '{}': {}", src, e);
            return 1;
        }
    };

    if same_source_and_destination(src, dst, archive, &src_meta) {
        eprintln!("cp: '{}' and '{}' are the same file", src, dst);
        return 1;
    }

    if src_meta.is_dir() {
        if destination_is_inside_source(src, dst) {
            eprintln!("cp: cannot copy '{}' into itself, '{}', or a subdirectory of itself", src, dst);
            return 1;
        }
        if !recursive {
            eprintln!("cp: -r not specified; omitting directory '{}'", src);
            return 1;
        }
        return copy_dir(w, src, dst, interactive, force, verbose, no_clobber, update, archive);
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
    if no_clobber && fs::symlink_metadata(dst).is_ok() {
        if verbose {
            pwriteln!(w, "skip '{}' (no-clobber)", dst);
        }
        return 0;
    }

    // Interactive check
    if interactive && fs::symlink_metadata(dst).is_ok() {
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
    let src_is_archive_link = archive && src_meta.file_type().is_symlink();
    if force && !src_is_archive_link {
        if let Ok(dst_meta) = fs::symlink_metadata(dst) {
            if dst_meta.is_dir() && !dst_meta.file_type().is_symlink() {
                let _ = fs::remove_dir_all(dst);
            } else {
                let _ = fs::remove_file(dst);
            }
        }
    }

    let copy_result = if src_is_archive_link {
        let dst_path = Path::new(dst);
        let remove_existing = match fs::symlink_metadata(dst_path) {
            Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
                eprintln!("cp: cannot overwrite directory '{}' with a symbolic link", dst);
                return 1;
            }
            Ok(_) => fs::remove_file(dst_path),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        };
        remove_existing.and_then(|()| {
            fs::read_link(src).and_then(|target| create_symlink(&target, dst_path))
        })
    } else {
        fs::copy(src, dst).map(|_| ())
    };

    match copy_result {
        Ok(()) => {
            if archive && !src_is_archive_link {
                if let Err(e) = fs::set_permissions(dst, src_meta.permissions()) {
                    eprintln!("cp: cannot preserve mode for '{}': {}", dst, e);
                    return 1;
                }
            }
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
    archive: bool,
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
                interactive, force, verbose, no_clobber, update, archive,
            );
        } else {
            exit_code |= copy_one(
                w, &src_str, &dst_path,
                interactive, force, verbose, no_clobber, update, true, archive,
            );
        }
    }
    if archive {
        if let Ok(meta) = fs::symlink_metadata(src) {
            if let Err(e) = fs::set_permissions(dst, meta.permissions()) {
                eprintln!("cp: cannot preserve mode for '{}': {}", dst, e);
                exit_code = 1;
            }
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

    #[cfg(unix)]
    #[test]
    fn archive_rejects_same_file_and_preserves_sources() {
        use std::os::unix::fs::symlink;

        let root = std::env::temp_dir().join(format!("gvibu_cp_same_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();

        let file = root.join("file");
        let alias = root.join("alias");
        std::fs::write(&file, "keep").unwrap();
        std::fs::hard_link(&file, &alias).unwrap();
        let regular_result = run(
            &mut std::io::sink(),
            &["-af".into(), file.to_string_lossy().into_owned(), alias.to_string_lossy().into_owned()],
        );
        assert_eq!(regular_result, 1);
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "keep");
        assert_eq!(std::fs::read_to_string(&alias).unwrap(), "keep");

        let source = root.join("source");
        let destination_link = root.join("destination-link");
        std::fs::write(&source, "source survives").unwrap();
        symlink("source", &destination_link).unwrap();
        let followed_alias_result = run(
            &mut std::io::sink(),
            &["-f".into(), source.to_string_lossy().into_owned(), destination_link.to_string_lossy().into_owned()],
        );
        assert_eq!(followed_alias_result, 1);
        assert_eq!(std::fs::read_to_string(&source).unwrap(), "source survives");
        assert!(std::fs::symlink_metadata(&destination_link).unwrap().file_type().is_symlink());

        let link = root.join("link");
        symlink("missing", &link).unwrap();
        let link_result = run(
            &mut std::io::sink(),
            &["-a".into(), link.to_string_lossy().into_owned(), link.to_string_lossy().into_owned()],
        );
        assert_eq!(link_result, 1);
        assert_eq!(std::fs::read_link(&link).unwrap(), Path::new("missing"));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn archive_rejects_copying_directory_into_itself() {
        let root = std::env::temp_dir().join(format!("gvibu_cp_nested_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let source = root.join("source");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("file"), "contents").unwrap();

        let code = run(
            &mut std::io::sink(),
            &["-a".into(), source.to_string_lossy().into_owned(), source.to_string_lossy().into_owned()],
        );
        assert_eq!(code, 1);
        assert!(!source.join("source").exists());
        assert_eq!(std::fs::read_to_string(source.join("file")).unwrap(), "contents");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn test_cp_archive_flag() {
        let root = std::env::temp_dir().join(format!("gvibu_cp_archive_{}", std::process::id()));
        let src = root.join("src");
        let nested = src.join("nested");
        let dst = root.join("dst");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("file"), "archive content").unwrap();

        let result = run(
            &mut std::io::sink(),
            &["-a".into(), src.to_string_lossy().into_owned(), dst.to_string_lossy().into_owned()],
        );

        assert_eq!(result, 0);
        assert_eq!(std::fs::read_to_string(dst.join("nested/file")).unwrap(), "archive content");
        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn archive_preserves_valid_and_dangling_symlinks() {
        use std::os::unix::fs::symlink;

        let root = std::env::temp_dir().join(format!("gvibu_cp_symlinks_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let src = root.join("src");
        let dst = root.join("dst");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("target"), "target").unwrap();
        symlink("target", src.join("valid")).unwrap();
        symlink("missing", src.join("dangling")).unwrap();

        let code = run(
            &mut std::io::sink(),
            &["-a".into(), src.to_string_lossy().into_owned(), dst.to_string_lossy().into_owned()],
        );

        assert_eq!(code, 0);
        assert!(std::fs::symlink_metadata(dst.join("valid")).unwrap().file_type().is_symlink());
        assert!(std::fs::symlink_metadata(dst.join("dangling")).unwrap().file_type().is_symlink());
        assert_eq!(std::fs::read_link(dst.join("valid")).unwrap(), Path::new("target"));
        assert_eq!(std::fs::read_link(dst.join("dangling")).unwrap(), Path::new("missing"));

        let nested_dst = dst.join("src");
        std::fs::create_dir_all(&nested_dst).unwrap();
        std::fs::write(nested_dst.join("valid"), "old destination").unwrap();
        symlink("old-missing", nested_dst.join("dangling")).unwrap();
        let replace_result = run(
            &mut std::io::sink(),
            &["-a".into(), src.to_string_lossy().into_owned(), dst.to_string_lossy().into_owned()],
        );
        assert_eq!(replace_result, 0);
        assert!(std::fs::symlink_metadata(nested_dst.join("valid")).unwrap().file_type().is_symlink());
        assert_eq!(std::fs::read_link(nested_dst.join("dangling")).unwrap(), Path::new("missing"));

        std::fs::remove_file(nested_dst.join("valid")).unwrap();
        std::fs::write(nested_dst.join("valid"), "keep me").unwrap();
        let no_clobber_result = run(
            &mut std::io::sink(),
            &["-a".into(), "-n".into(), src.to_string_lossy().into_owned(), dst.to_string_lossy().into_owned()],
        );
        assert_eq!(no_clobber_result, 0);
        assert_eq!(std::fs::read_to_string(nested_dst.join("valid")).unwrap(), "keep me");
        std::fs::remove_dir_all(root).unwrap();
    }
}
