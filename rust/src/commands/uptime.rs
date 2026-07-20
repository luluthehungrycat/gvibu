use std::io::Write;

/// Linux: read uptime from /proc/uptime
#[cfg(target_os = "linux")]
fn get_uptime_seconds() -> u64 {
    let uptime_str = std::fs::read_to_string("/proc/uptime").unwrap_or_default();
    uptime_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0) as u64
}

/// macOS: use sysctlbyname("kern.boottime") to get boot time,
/// then compute uptime from wall clock.
#[cfg(not(target_os = "linux"))]
fn get_uptime_seconds() -> u64 {
    // SAFETY: `libc::timeval` is a POD struct of two integer fields whose
    // all-zero byte pattern is a valid instance, so `mem::zeroed()` is sound;
    // the value is only read after `sysctlbyname` returns 0 (success) and
    // populates the buffer.
    let mut boottime: libc::timeval = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<libc::timeval>() as libc::size_t;
    // SAFETY: the first argument is a pointer to a NUL-terminated byte string
    // literal (`b"kern.boottime\0"`) with `'static` lifetime, the second is a
    // valid pointer to a stack-allocated `libc::timeval` of the correct size
    // (matching `len`), the length pointer is initialised to that size, and
    // the old-value pointer is NULL with old-value size 0 (no read). The call
    // therefore satisfies the `sysctlbyname` FFI contract.
    let ret = unsafe {
        libc::sysctlbyname(
            b"kern.boottime\0".as_ptr() as *const libc::c_char,
            &mut boottime as *mut _ as *mut libc::c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if ret != 0 {
        return 0;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let boot_secs = boottime.tv_sec as u64;
    let now_secs = now.as_secs();
    now_secs.saturating_sub(boot_secs)
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let seconds = get_uptime_seconds();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        let _ = writeln!(
            stdout,
            "up {} day{}, {} hour{}, {} minute{}",
            days,
            if days == 1 { "" } else { "s" },
            hours,
            if hours == 1 { "" } else { "s" },
            minutes,
            if minutes == 1 { "" } else { "s" }
        );
    } else if hours > 0 {
        let _ = writeln!(
            stdout,
            "up {} hour{}, {} minute{}",
            hours,
            if hours == 1 { "" } else { "s" },
            minutes,
            if minutes == 1 { "" } else { "s" }
        );
    } else {
        let m = if minutes == 0 { 1 } else { minutes };
        let _ = writeln!(stdout, "up {} minute{}", m, if m == 1 { "" } else { "s" });
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_output() {
        // Just ensure it doesn't panic
        let mut buf = Vec::new();
        run(&mut buf, &[]);
        assert!(!buf.is_empty());
    }
}
