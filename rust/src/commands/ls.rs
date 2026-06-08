/// ls: list directory contents.
use std::fs::{self, Metadata};
use std::io::Write;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::UNIX_EPOCH;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut show_all = false;
    let mut show_almost_all = false;
    let mut long = false;
    let mut human = false;
    let mut sort_time = false;
    let mut reverse = false;
    let mut sort_size = false;
    let mut one_per_line = true; // default to 1-column
    let mut paths: Vec<&str> = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-a" => show_all = true,
            "-A" => show_almost_all = true,
            "-l" => long = true,
            "-h" => human = true,
            "-t" => sort_time = true,
            "-r" => reverse = true,
            "-S" => sort_size = true,
            "-1" => one_per_line = true,
            "--" => break,
            s if s.starts_with('-') && s.len() > 1 => {
                // Handle bundled flags like -la, -ltr
                for c in s[1..].chars() {
                    match c {
                        'a' => show_all = true,
                        'A' => show_almost_all = true,
                        'l' => long = true,
                        'h' => human = true,
                        't' => sort_time = true,
                        'r' => reverse = true,
                        'S' => sort_size = true,
                        '1' => one_per_line = true,
                        _ => {
                            eprintln!("ls: invalid option: -{}", c);
                            return 1;
                        }
                    }
                }
            }
            _ => paths.push(arg.as_str()),
        }
    }

    if paths.is_empty() {
        paths.push(".");
    }

    let mut exit_code = 0;
    let mut first = true;

    for path in paths {
        match fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() {
                    if !first {
                        pwriteln!(w);
                    }
                    if paths.len() > 1 {
                        pwriteln!(w, "{}:", path);
                    }
                    first = false;
                    exit_code |= list_directory(w, path, long, show_all, show_almost_all, human, sort_time, reverse, sort_size);
                    if paths.len() > 1 {
                        pwriteln!(w);
                    }
                } else {
                    if !first {
                        pwriteln!(w);
                    }
                    first = false;
                    if long {
                        pwriteln!(w, "{}", format_long(path, &meta, human));
                    } else {
                        pwriteln!(w, "{}", display_name(path));
                    }
                }
            }
            Err(e) => {
                eprintln!("ls: cannot access '{}': {}", path, e);
                exit_code = 1;
            }
        }
    }

    exit_code
}

fn list_directory(
    w: &mut dyn Write,
    dir: &str,
    long: bool,
    show_all: bool,
    show_almost_all: bool,
    human: bool,
    sort_time: bool,
    reverse: bool,
    sort_size: bool,
) -> i32 {
    let entries = match fs::read_dir(dir) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ls: cannot read directory '{}': {}", dir, e);
            return 1;
        }
    };

    let mut items: Vec<(String, Metadata)> = Vec::new();
    for entry in entries {
        match entry {
            Ok(e) => {
                let name = e.file_name().to_string_lossy().to_string();
                // Filter hidden files
                if name.starts_with('.') {
                    if show_all {
                        // include everything
                    } else if show_almost_all && name != "." && name != ".." {
                        // include
                    } else {
                        continue;
                    }
                }
                match e.metadata() {
                    Ok(m) => items.push((name, m)),
                    Err(_) => items.push((name, unsafe { std::mem::zeroed() })),
                }
            }
            Err(_) => {}
        }
    }

    // Sort
    if sort_time {
        items.sort_by(|a, b| b.1.mtime().cmp(&a.1.mtime()));
    } else if sort_size {
        items.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    } else {
        items.sort_by(|a, b| a.0.cmp(&b.0));
    }

    if reverse {
        items.reverse();
    }

    if long {
        for (name, meta) in &items {
            pwriteln!(w, "{}", format_long(name, meta, human));
        }
    } else {
        for (name, _meta) in &items {
            pwriteln!(w, "{}", name);
        }
    }

    0
}

fn format_long(name: &str, meta: &Metadata, human: bool) -> String {
    let file_type = if meta.is_dir() { 'd' } else if meta.file_type().is_symlink() { 'l' } else { '-' };
    let mode = meta.permissions().mode();
    let perms = format_mode(mode);
    let nlink = meta.nlink();
    let size = if human {
        human_size(meta.len())
    } else {
        meta.len().to_string()
    };
    let mtime = format_time(meta.mtime());
    format!("{}{} {:>2} {} {} {} {}", file_type, perms, nlink, "", "", size, mtime, name)
}

fn format_mode(mode: u32) -> String {
    let perm = |owner: u32, group: u32, other: u32| -> String {
        let r = |bit, c| if mode & bit != 0 { c } else { '-' };
        format!(
            "{}{}{}",
            r(owner, 'r'),
            r(group, 'w'),
            r(other, 'x')
        )
    };
    format!(
        "{}{}{}",
        perm(0o400, 0o200, 0o100),
        perm(0o040, 0o020, 0o010),
        perm(0o004, 0o002, 0o001)
    )
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["", "K", "M", "G", "T"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{}", bytes)
    } else {
        format!("{:.1}{}", size, UNITS[unit_idx])
    }
}

fn format_time(secs: i64) -> String {
    // Simple format: "Jan  2 12:34" style (6-month cutoff like GNU ls)
    let secs = if secs >= 0 { secs as u64 } else { 0 };
    let dur = std::time::Duration::from_secs(secs);
    let time = UNIX_EPOCH + dur;

    // Use chrono-like manual formatting via local time
    // For simplicity, just show the raw timestamp parts
    // We'll do a basic approach
    let (year, month, day, hour, min) = unix_to_ymd_hm(secs as i64);
    let now_secs = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let (now_year, _, _, _, _) = unix_to_ymd_hm(now_secs);

    let month_str = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        _ => "Dec",
    };

    if year == now_year {
        format!("{} {:>2} {:02}:{:02}", month_str, day, hour, min)
    } else {
        format!("{} {:>2}  {:>4}", month_str, day, year)
    }
}

fn unix_to_ymd_hm(secs: i64) -> (i64, i64, i64, i64, i64) {
    // Simple algorithm for seconds since epoch
    let mut s = secs;
    let mut min = (s / 60) % 60;
    if min < 0 { min += 60; s -= 60 * 60; }
    let hours = (s / 3600) % 24;
    let days = s / 86400;

    // Year/month/day from days since epoch
    let mut y = 1970i64;
    let mut remaining = days;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let month_days = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut m = 0i64;
    for (i, &md) in month_days.iter().enumerate() {
        if remaining < md {
            m = (i + 1) as i64;
            break;
        }
        remaining -= md;
    }
    if m == 0 {
        m = 12;
    }
    let day = remaining + 1;

    (y, m, day, hours, min)
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn display_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls_dev_null() {
        // /dev/null is a file, not a directory - should list it
        let result = run(&mut std::io::sink(), &["/dev/null"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ls_invalid_option() {
        let result = run(&mut std::io::sink(), &["-x"]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_ls_current_dir() {
        let result = run(&mut std::io::sink(), &[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ls_a_flag() {
        let result = run(&mut std::io::sink(), &["-a"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ls_l_flag() {
        let result = run(&mut std::io::sink(), &["-l"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ls_bundled_flags() {
        let result = run(&mut std::io::sink(), &["-la"]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_ls_nonexistent() {
        let result = run(&mut std::io::sink(), &["/nonexistent_ls_test_xyz"]);
        assert_eq!(result, 1);
    }
}
