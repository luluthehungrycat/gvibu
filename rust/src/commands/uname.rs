/// uname: print system information.
/// Linux: reads from /proc/sys/kernel/*
/// macOS/other: uses POSIX libc::uname()
use std::io::Write;

#[cfg(target_os = "linux")]
fn get_kernel_info() -> (String, String, String, &'static str) {
    fn read_proc(path: &str) -> String {
        std::fs::read_to_string(path)
            .map(|s| s.trim().to_string())
            .unwrap_or_default()
    }
    let sysname = read_proc("/proc/sys/kernel/ostype");
    let nodename = read_proc("/proc/sys/kernel/hostname");
    let release = read_proc("/proc/sys/kernel/osrelease");
    let machine = std::env::consts::ARCH;
    (sysname, nodename, release, machine)
}

#[cfg(not(target_os = "linux"))]
fn get_kernel_info() -> (String, String, String, &'static str) {
    // SAFETY: `libc::utsname` is a POD struct whose all-zero byte pattern is a
    // valid instance (no niche/provenance requirements), so `mem::zeroed()` is
    // sound; the kernel will overwrite every field on the subsequent `uname`
    // call, and we never read uninitialised memory.
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    // SAFETY: `uts` is a stack-allocated, properly aligned `libc::utsname`
    // whose size matches the kernel's expectation; its address is valid for
    // the duration of the call, and `libc::uname` is safe to invoke on any
    // thread with no additional preconditions.
    let ret = unsafe { libc::uname(&mut uts) };
    if ret != 0 {
        return ("Unknown".into(), "Unknown".into(), "Unknown".into(), std::env::consts::ARCH);
    }
    fn from_cstr(arr: &[i8]) -> String {
        let bytes: Vec<u8> = arr.iter().take_while(|&&b| b != 0).map(|&b| b as u8).collect();
        String::from_utf8_lossy(&bytes).to_string()
    }
    let sysname = from_cstr(&uts.sysname);
    let nodename = from_cstr(&uts.nodename);
    let release = from_cstr(&uts.release);
    let machine = std::env::consts::ARCH;
    (sysname, nodename, release, machine)
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut flags = String::new();
    let mut all_flag = false;

    for arg in args {
        if arg == "--" { break; }
        if let Some(chars) = arg.strip_prefix('-') {
            for ch in chars.chars() {
                match ch {
                    'a' => all_flag = true,
                    's' | 'n' | 'r' | 'm' => flags.push(ch),
                    _ => {
                        eprintln!("uname: invalid option: -{}", ch);
                        return 1;
                    }
                }
            }
        } else {
            eprintln!("uname: invalid option: {}", arg);
            return 1;
        }
    }

    if flags.is_empty() && !all_flag {
        flags.push('s');
    }

    if all_flag {
        flags = String::from("snrm");
    }

    let (sysname, nodename, release, machine) = get_kernel_info();
    let sysname = if sysname.is_empty() { "Linux" } else { &sysname };

    let mut parts = Vec::new();
    for flag in flags.chars() {
        match flag {
            's' => parts.push(sysname),
            'n' => parts.push(if nodename.is_empty() { "(none)" } else { &nodename }),
            'r' => parts.push(if release.is_empty() { "(unknown)" } else { &release }),
            'm' => parts.push(machine),
            _ => {}
        }
    }

    let _ = writeln!(stdout, "{}", parts.join(" "));
    0
}
