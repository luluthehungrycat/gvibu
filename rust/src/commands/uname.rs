/// uname: print system information.
/// Reads from /proc/sys/kernel/* instead of running subprocess uname.
use std::fs;

fn read_proc(path: &str) -> String {
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

pub fn run(args: &[String]) -> i32 {
    let mut flags = String::new();
    let mut all_flag = false;

    for arg in args {
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

    let sysname = read_proc("/proc/sys/kernel/ostype");
    let nodename = read_proc("/proc/sys/kernel/hostname");
    let release = read_proc("/proc/sys/kernel/osrelease");
    let machine = std::env::consts::ARCH;

    let mut parts = Vec::new();
    for flag in flags.chars() {
        match flag {
            's' => parts.push(if sysname.is_empty() { "Linux" } else { &sysname }),
            'n' => parts.push(if nodename.is_empty() { "(none)" } else { &nodename }),
            'r' => parts.push(if release.is_empty() { "(unknown)" } else { &release }),
            'm' => parts.push(machine),
            _ => {}
        }
    }

    println!("{}", parts.join(" "));
    0
}
