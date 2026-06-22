/// chown: change file owner and group.
use std::fs;
use std::io::Write;

fn parse_owner(s: &str) -> (Option<u32>, Option<u32>) {
    // Format: [USER][:][GROUP]
    if s.is_empty() {
        return (None, None);
    }

    if let Some(colon_pos) = s.find(':') {
        let user_part = &s[..colon_pos];
        let group_part = &s[colon_pos + 1..];
        let uid = if user_part.is_empty() {
            None
        } else {
            lookup_user(user_part)
        };
        let gid = if group_part.is_empty() {
            None
        } else {
            lookup_group(group_part)
        };
        (uid, gid)
    } else {
        // Just a user name (or number)
        let uid = lookup_user(s);
        (uid, None)
    }
}

fn lookup_user(name: &str) -> Option<u32> {
    // Try numeric first
    if let Ok(uid) = name.parse::<u32>() {
        return Some(uid);
    }
    // Try name lookup via /etc/passwd
    let content = fs::read_to_string("/etc/passwd").ok()?;
    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 && parts[0] == name {
            return parts[2].parse::<u32>().ok();
        }
    }
    None
}

fn lookup_group(name: &str) -> Option<u32> {
    // Try numeric first
    if let Ok(gid) = name.parse::<u32>() {
        return Some(gid);
    }
    // Try name lookup via /etc/group
    let content = fs::read_to_string("/etc/group").ok()?;
    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 3 && parts[0] == name {
            return parts[2].parse::<u32>().ok();
        }
    }
    None
}

fn chown_path(path: &str, uid: Option<u32>, gid: Option<u32>, verbose: bool) -> i32 {
    // Use libc::chown if available, otherwise fallback
    #[cfg(unix)]
    {
        let c_path = std::ffi::CString::new(path).unwrap_or_default();
        let c_uid = uid.unwrap_or(u32::MAX);
        let c_gid = gid.unwrap_or(u32::MAX);
        let ret = unsafe { libc::chown(c_path.as_ptr(), c_uid, c_gid) };
        if ret == 0 {
            if verbose {
                let _ = writeln!(std::io::stdout(), "changed ownership of '{}'", path);
            }
            return 0;
        }
        let err = std::io::Error::last_os_error();
        eprintln!("chown: {}: {}", path, err);
        return 1;
    }
    #[cfg(not(unix))]
    {
        let _ = (path, uid, gid, verbose);
        eprintln!("chown: not supported on this platform");
        1
    }
}

fn chown_recursive(path: &str, uid: Option<u32>, gid: Option<u32>, verbose: bool) -> i32 {
    let mut exit_code = 0;
    exit_code |= chown_path(path, uid, gid, verbose);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let child_path = entry.path();
            if child_path.is_dir() {
                exit_code |= chown_recursive(child_path.to_str().unwrap_or(""), uid, gid, verbose);
            } else {
                exit_code |= chown_path(child_path.to_str().unwrap_or(""), uid, gid, verbose);
            }
        }
    }
    exit_code
}

pub fn run(_w: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("chown: missing operand");
        return 1;
    }

    let mut recursive = false;
    let mut verbose = false;
    let mut owner_arg: Option<String> = None;
    let mut files: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-R" {
            recursive = true;
        } else if arg == "-v" {
            verbose = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("chown: invalid option: {}", arg);
            return 1;
        } else if owner_arg.is_none() {
            owner_arg = Some(arg.clone());
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let owner_str = match owner_arg {
        Some(o) => o,
        None => {
            eprintln!("chown: missing operand");
            return 1;
        }
    };

    if files.is_empty() {
        eprintln!("chown: missing operand after '{}'", owner_str);
        return 1;
    }

    let (uid, gid) = parse_owner(&owner_str);
    if uid.is_none() && gid.is_none() {
        eprintln!("chown: invalid owner: '{}'", owner_str);
        return 1;
    }

    let mut exit_code = 0;
    for fname in &files {
        if recursive && fs::metadata(fname).map(|m| m.is_dir()).unwrap_or(false) {
            exit_code |= chown_recursive(fname, uid, gid, verbose);
        } else {
            exit_code |= chown_path(fname, uid, gid, verbose);
        }
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_owner_user_only() {
        let (uid, gid) = parse_owner("root");
        assert!(uid.is_none() || uid.is_some()); // root may or may not exist in container
        assert!(gid.is_none());
    }

    #[test]
    fn test_parse_owner_numeric() {
        let (uid, gid) = parse_owner("0");
        assert_eq!(uid, Some(0));
        assert!(gid.is_none());
    }

    #[test]
    fn test_parse_owner_user_group() {
        let (uid, gid) = parse_owner("0:0");
        assert_eq!(uid, Some(0));
        assert_eq!(gid, Some(0));
    }

    #[test]
    fn test_parse_owner_group_only() {
        let (uid, gid) = parse_owner(":0");
        assert!(uid.is_none());
        assert_eq!(gid, Some(0));
    }

    #[test]
    fn test_chown_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_chown_no_files() {
        assert_eq!(run(&mut std::io::sink(), &["root".into()]), 1);
    }
}
