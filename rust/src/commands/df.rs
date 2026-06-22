/// df: report file system space usage.
use std::ffi::CString;
use std::fs;
use std::io::Write;
use crate::pwriteln;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut human = false;
    let mut show_type = false;
    let mut explicit_paths: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => human = true,
            "-T" => show_type = true,
            "--" => { break; }
            arg if arg.starts_with('-') && arg.len() > 1 => {
                for c in arg[1..].chars() {
                    match c {
                        'h' => human = true,
                        'T' => show_type = true,
                        _ => {
                            pwriteln!(w, "df: invalid option: -{}", c);
                            return 1;
                        }
                    }
                }
            }
            p => explicit_paths.push(p),
        }
        i += 1;
    }

    // Collect mount points
    let mounts = match read_mounts() {
        Ok(m) => m,
        Err(e) => {
            pwriteln!(w, "df: {}", e);
            return 1;
        }
    };

    // Print header
    if show_type {
        pwriteln!(w, "Filesystem     Type       1K-blocks      Used    Available  Use% Mounted on");
    } else {
        pwriteln!(w, "Filesystem     1K-blocks      Used    Available  Use% Mounted on");
    }

    let mut exit_code = 0;

    for mount in &mounts {
        if !explicit_paths.is_empty() {
            if !explicit_paths.iter().any(|p| mount.mount_point == *p || mount.fs_file.starts_with(p)) {
                continue;
            }
        }

        if mount.fs_type == "rootfs" || mount.fs_type == "proc" || mount.fs_type == "sysfs" || mount.fs_type == "cgroup" || mount.fs_type == "devpts" || mount.fs_type == "devtmpfs" || mount.fs_type == "tmpfs" || mount.fs_type == "pstore" || mount.fs_type == "securityfs" || mount.fs_type == "hugetlbfs" || mount.fs_type == "mqueue" || mount.fs_type == "debugfs" || mount.fs_type == "tracefs" || mount.fs_type == "configfs" || mount.fs_type == "efivarfs" || mount.fs_type == "fusectl" {
            continue;
        }

        match get_fs_stats(&mount.mount_point) {
            Some((total_1k, used_1k, avail_1k, use_pct)) => {
                if human {
                    let total_h = human_size(total_1k * 1024);
                    let used_h = human_size(used_1k * 1024);
                    let avail_h = human_size(avail_1k * 1024);
                    if show_type {
                        let _ = writeln!(
                            w,
                            "{:<14} {:<11} {:>5} {:>8} {:>8} {:>3}% {}",
                            mount.fs_file,
                            mount.fs_type,
                            total_h,
                            used_h,
                            avail_h,
                            use_pct,
                            mount.mount_point
                        );
                    } else {
                        let _ = writeln!(
                            w,
                            "{:<14} {:>5} {:>8} {:>8} {:>3}% {}",
                            mount.fs_file,
                            total_h,
                            used_h,
                            avail_h,
                            use_pct,
                            mount.mount_point
                        );
                    }
                } else {
                    if show_type {
                        let _ = writeln!(
                            w,
                            "{:<14} {:<11} {:>8} {:>8} {:>8} {:>3}% {}",
                            mount.fs_file,
                            mount.fs_type,
                            total_1k,
                            used_1k,
                            avail_1k,
                            use_pct,
                            mount.mount_point
                        );
                    } else {
                        let _ = writeln!(
                            w,
                            "{:<14} {:>8} {:>8} {:>8} {:>3}% {}",
                            mount.fs_file,
                            total_1k,
                            used_1k,
                            avail_1k,
                            use_pct,
                            mount.mount_point
                        );
                    }
                }
            }
            None => {
                exit_code = 1;
            }
        }
    }

    exit_code
}

struct MountInfo {
    fs_file: String,
    mount_point: String,
    fs_type: String,
}

fn read_mounts() -> Result<Vec<MountInfo>, String> {
    let content = fs::read_to_string("/proc/mounts").map_err(|e| format!("cannot read /proc/mounts: {}", e))?;
    let mut mounts = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            mounts.push(MountInfo {
                fs_file: parts[0].to_string(),
                mount_point: parts[1].to_string(),
                fs_type: parts[2].to_string(),
            });
        }
    }

    Ok(mounts)
}

fn get_fs_stats(path: &str) -> Option<(u64, u64, u64, u64)> {
    let cpath = CString::new(path).ok()?;
    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    let ret = unsafe { libc::statvfs(cpath.as_ptr(), stat.as_mut_ptr()) };
    if ret != 0 {
        return None;
    }
    let stat = unsafe { stat.assume_init() };

    let frsize = stat.f_frsize as u64;
    let blocks = stat.f_blocks as u64;
    let bfree = stat.f_bfree as u64;
    let bavail = stat.f_bavail as u64;

    let total_1k = (blocks * frsize) / 1024;
    let avail_1k = (bavail * frsize) / 1024;
    let used_1k = total_1k.saturating_sub((bfree * frsize) / 1024);
    let use_pct = if total_1k > 0 {
        (used_1k * 100) / total_1k
    } else {
        0
    };

    Some((total_1k, used_1k, avail_1k, use_pct))
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{}", bytes)
    } else if size >= 100.0 {
        format!("{:.0}{}", size, UNITS[unit_idx])
    } else if size >= 10.0 {
        format!("{:.1}{}", size, UNITS[unit_idx])
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_human_size_zero() {
        assert_eq!(human_size(0), "0");
    }

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human_size(500), "500");
    }

    #[test]
    fn test_human_size_kb() {
        assert_eq!(human_size(2048), "2.0K");
    }

    #[test]
    fn test_human_size_mb() {
        assert_eq!(human_size(1048576 * 2), "2.0M");
    }

    #[test]
    fn test_bundled_flags() {
        let result = run(&mut std::io::sink(), &["-hT".into()]);
        assert!(result == 0 || result == 1);
    }

    #[test]
    fn test_no_args() {
        // Just check it doesn't crash - relies on /proc/mounts
        let result = run(&mut std::io::sink(), &[]);
        assert!(result == 0 || result == 1);
    }
}
