/// touch: update file timestamps or create empty files.
use std::fs;
use std::io::{self, Write};

#[cfg(not(unix))]
use std::fs::FileTimes;

#[cfg(unix)]
fn update_times(path: &str, access: bool, modification: bool) -> io::Result<()> {
    use std::ffi::CString;

    let path = CString::new(path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains NUL"))?;
    let now = libc::timespec {
        tv_sec: 0,
        tv_nsec: libc::UTIME_NOW,
    };
    let omit = libc::timespec {
        tv_sec: 0,
        tv_nsec: libc::UTIME_OMIT,
    };
    let times = [
        if access { now } else { omit },
        if modification { now } else { omit },
    ];

    let result = unsafe { libc::utimensat(libc::AT_FDCWD, path.as_ptr(), times.as_ptr(), 0) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(not(unix))]
fn update_times(path: &str, access: bool, modification: bool) -> io::Result<()> {
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    let now = std::time::SystemTime::now();
    let mut times = FileTimes::new();
    if access {
        times = times.set_accessed(now);
    }
    if modification {
        times = times.set_modified(now);
    }
    file.set_times(times)
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    if args.is_empty() {
        eprintln!("touch: usage: touch FILE...");
        return 2;
    }

    let mut flag_a = false;
    let mut flag_m = false;
    let mut files: Vec<&str> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" {
            break;
        }
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'a' => flag_a = true,
                    'm' => flag_m = true,
                    _ => {
                        eprintln!("touch: invalid option: -{}", ch);
                        return 1;
                    }
                }
            }
        } else {
            files.push(arg.as_str());
        }
        i += 1;
    }

    if files.is_empty() {
        eprintln!("touch: usage: touch FILE...");
        return 2;
    }

    // Default: update both if neither specified
    if !flag_a && !flag_m {
        flag_a = true;
        flag_m = true;
    }

    let mut exit_code = 0;

    for fname in files {
        // Open to create if missing (append mode).
        match fs::OpenOptions::new().append(true).create(true).open(fname) {
            Ok(mut file) => {
                let _ = file.flush();
                if let Err(e) = update_times(fname, flag_a, flag_m) {
                    eprintln!("touch: error setting times: {}", e);
                    exit_code = 1;
                }
            }
            Err(e) => {
                eprintln!("touch: {}: {}", fname, e);
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
    fn test_touch_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 2);
    }

    #[test]
    fn test_touch_invalid_option() {
        assert_eq!(
            run(&mut std::io::sink(), &["-x".into(), "/tmp/test".into()]),
            1
        );
    }

    #[test]
    fn test_touch_flag_a() {
        let tmp = std::env::temp_dir().join("gvibu_touch_test_a");
        let path = tmp.to_str().unwrap().to_string();
        let _ = std::fs::remove_file(&path);
        assert_eq!(
            run(&mut std::io::sink(), &["-a".into(), path.clone().into()]),
            0
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_touch_flag_m() {
        let tmp = std::env::temp_dir().join("gvibu_touch_test_m");
        let path = tmp.to_str().unwrap().to_string();
        assert_eq!(
            run(&mut std::io::sink(), &["-m".into(), path.clone().into()]),
            0
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_touch_flag_am() {
        let tmp = std::env::temp_dir().join("gvibu_touch_test_am");
        let path = tmp.to_str().unwrap().to_string();
        assert_eq!(
            run(&mut std::io::sink(), &["-am".into(), path.clone().into()]),
            0
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_touch_no_flags() {
        let tmp = std::env::temp_dir().join("gvibu_touch_test_default");
        let path = tmp.to_str().unwrap().to_string();
        assert_eq!(run(&mut std::io::sink(), &[path.clone().into()]), 0);
        let _ = std::fs::remove_file(&path);
    }
}
