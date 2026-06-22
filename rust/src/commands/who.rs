/// who: show who is logged in.
use std::io::Write;

// ── Linux implementation (reads /var/run/utmp) ──
#[cfg(target_os = "linux")]
mod platform {
    use std::fs;
    use std::io::Write;

    // utmp record structure (Linux x86_64)
    #[repr(C)]
    struct Utmp {
        ut_type: i16,
        _pad: i16,
        ut_pid: i32,
        ut_line: [u8; 32],
        ut_id: [u8; 4],
        ut_user: [u8; 32],
        ut_host: [u8; 256],
        ut_exit: [u8; 8],
        ut_session: i32,
        _pad2: [u8; 4],
        ut_tv: [u8; 8],
        ut_addr_v6: [u8; 16],
        _reserved: [u8; 20],
    }

    const USER_PROCESS: i16 = 7;
    const LOGIN_PROCESS: i16 = 6;

    fn read_cstr(bytes: &[u8]) -> String {
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        String::from_utf8_lossy(&bytes[..end]).to_string()
    }

    fn is_leap(year: i64) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
        for arg in args {
            if arg == "--" { break; }
            if arg.starts_with('-') && arg.len() > 1 {
                eprintln!("who: invalid option: {}", arg);
                return 1;
            }
        }

        let utmp_paths = ["/var/run/utmp", "/run/utmp", "/var/log/wtmp"];
        let mut path = None;
        for p in &utmp_paths {
            if fs::metadata(p).is_ok() {
                path = Some(p.to_string());
                break;
            }
        }
        let path = match path {
            Some(p) => p,
            None => {
                eprintln!("who: cannot find utmp file");
                return 1;
            }
        };

        let data = match fs::read(&path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("who: {}: {}", path, e);
                return 1;
            }
        };

        let record_size = std::mem::size_of::<Utmp>();
        let mut found = false;

        for chunk in data.chunks(record_size) {
            if chunk.len() < record_size {
                break;
            }
            let ut_type = i16::from_ne_bytes([chunk[0], chunk[1]]);
            if ut_type != USER_PROCESS && ut_type != LOGIN_PROCESS {
                continue;
            }
            let user_bytes = &chunk[44..76];
            let line_bytes = &chunk[8..40];
            let host_bytes = &chunk[76..332];
            let tv_bytes = &chunk[356..364];

            let user = read_cstr(user_bytes);
            let line = read_cstr(line_bytes);
            let host = read_cstr(host_bytes);

            if user.is_empty() || user == "LOGIN" {
                continue;
            }

            let tv_sec = i32::from_ne_bytes([tv_bytes[0], tv_bytes[1], tv_bytes[2], tv_bytes[3]]);
            if tv_sec > 0 {
                let secs = tv_sec as i64;
                let days = secs / 86400;
                let time_of_day = secs % 86400;
                let hours = time_of_day / 3600;
                let minutes = (time_of_day % 3600) / 60;
                let seconds = time_of_day % 60;

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
                let months_days: [i64; 12] = if is_leap(y) {
                    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                } else {
                    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                };
                let mut m = 0i64;
                for (mi, &md) in months_days.iter().enumerate() {
                    if remaining < md {
                        m = mi as i64;
                        break;
                    }
                    remaining -= md;
                }
                let d = remaining + 1;
                let _months_short = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
                let _ = writeln!(stdout, "{:<8} {:<12} {}-{:02}-{:02} {:02}:{:02}:{:02} ({})",
                    user, line, y, m + 1, d, hours, minutes, seconds, host);
            } else {
                let _ = writeln!(stdout, "{:<8} {:<12}", user, line);
            }
            found = true;
        }

        if !found {
            let _ = writeln!(stdout, "no users logged in");
        }
        0
    }
}

// ── macOS implementation (uses getutxent API) ──
#[cfg(not(target_os = "linux"))]
mod platform {
    use std::io::Write;

    fn read_cstr(bytes: &[u8]) -> String {
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        String::from_utf8_lossy(&bytes[..end]).to_string()
    }

    pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
        for arg in args {
            if arg == "--" { break; }
            if arg.starts_with('-') && arg.len() > 1 {
                eprintln!("who: invalid option: {}", arg);
                return 1;
            }
        }

        let mut found = false;
        unsafe {
            libc::setutxent();
            loop {
                let ptr = libc::getutxent();
                if ptr.is_null() {
                    break;
                }
                let utx = &*ptr;
                if utx.ut_type != libc::USER_PROCESS {
                    continue;
                }
                let user = read_cstr(&utx.ut_user);
                let line = read_cstr(&utx.ut_line);
                let host = read_cstr(&utx.ut_host);

                if user.is_empty() {
                    continue;
                }

                let tv_sec = utx.ut_tv.tv_sec;
                if tv_sec > 0 {
                    let secs = tv_sec as i64;
                    let days = secs / 86400;
                    let time_of_day = secs % 86400;
                    let hours = time_of_day / 3600;
                    let minutes = (time_of_day % 3600) / 60;
                    let seconds = time_of_day % 60;

                    let mut y = 1970i64;
                    let mut remaining = days;
                    loop {
                        let days_in_year = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 { 366 } else { 365 };
                        if remaining < days_in_year {
                            break;
                        }
                        remaining -= days_in_year;
                        y += 1;
                    }
                    let months_days: [i64; 12] = if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
                        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                    } else {
                        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                    };
                    let mut m = 0i64;
                    for (mi, &md) in months_days.iter().enumerate() {
                        if remaining < md {
                            m = mi as i64;
                            break;
                        }
                        remaining -= md;
                    }
                    let d = remaining + 1;
                    let _months_short = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
                    let _ = writeln!(stdout, "{:<8} {:<12} {}-{:02}-{:02} {:02}:{:02}:{:02} ({})",
                        user, line, y, m + 1, d, hours, minutes, seconds, host);
                } else {
                    let _ = writeln!(stdout, "{:<8} {:<12}", user, line);
                }
                found = true;
            }
            libc::endutxent();
        }

        if !found {
            let _ = writeln!(stdout, "no users logged in");
        }
        0
    }
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    platform::run(stdout, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_who_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_who_no_args() {
        let code = run(&mut std::io::sink(), &[]);
        assert!(code == 0 || code == 1);
    }
}
