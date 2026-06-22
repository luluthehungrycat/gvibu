/// id: print user identity.
use std::io::Write;
use crate::pwrite;
use crate::pwriteln;

fn get_uid() -> u32 {
    unsafe { libc::getuid() }
}

fn get_gid() -> u32 {
    unsafe { libc::getgid() }
}

fn get_euid() -> u32 {
    unsafe { libc::geteuid() }
}

fn get_egid() -> u32 {
    unsafe { libc::getegid() }
}

fn get_groups() -> Vec<u32> {
    let ngroups = unsafe { libc::getgroups(0, std::ptr::null_mut()) };
    if ngroups <= 0 {
        return Vec::new();
    }
    let mut groups: Vec<u32> = vec![0; ngroups as usize];
    let ret = unsafe { libc::getgroups(ngroups, groups.as_mut_ptr()) };
    if ret < 0 {
        return Vec::new();
    }
    groups.truncate(ret as usize);
    groups
}

fn uid_to_name(uid: u32) -> String {
    // Try reading /etc/passwd
    if let Ok(content) = std::fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 2 {
                if let Ok(u) = parts[2].parse::<u32>() {
                    if u == uid {
                        return parts[0].to_string();
                    }
                }
            }
        }
    }
    uid.to_string()
}

fn gid_to_name(gid: u32) -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/group") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() > 2 {
                if let Ok(g) = parts[2].parse::<u32>() {
                    if g == gid {
                        return parts[0].to_string();
                    }
                }
            }
        }
    }
    gid.to_string()
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut flag_u = false;
    let mut flag_g = false;
    let mut flag_groups = false;
    let mut flag_name = false;
    let mut flag_real = false;

    for arg in args {
        if arg == "--" { break; }
        for ch in arg.chars().skip(1) {
            match ch {
                'u' => flag_u = true,
                'g' => flag_g = true,
                'G' => flag_groups = true,
                'n' => flag_name = true,
                'r' => flag_real = true,
                _ => {
                    eprintln!("id: invalid option: -{}", ch);
                    return 1;
                }
            }
        }
    }

    // Check for -n without -u/-g/-G
    if flag_name && !flag_u && !flag_g && !flag_groups {
        eprintln!("id: cannot print only names in default format");
        return 1;
    }

    // Default format
    if !flag_u && !flag_g && !flag_groups {
        let uid = get_euid();
        let gid = get_egid();
        let uname = uid_to_name(uid);
        let gname = gid_to_name(gid);
        let groups = get_groups();

        let _ = pwrite!(stdout, "uid={}({}) gid={}({})", uid, uname, gid, gname);

        let group_names: Vec<String> = groups.iter().map(|g| {
            let name = gid_to_name(*g);
            format!("{}({})", g, name)
        }).collect();
        let _ = pwrite!(stdout, " groups={}", group_names.join(","));
        let _ = pwriteln!(stdout);
        return 0;
    }

    // Print user ID
    if flag_u {
        let uid = if flag_real { get_uid() } else { get_euid() };
        if flag_name {
            let _ = pwriteln!(stdout, "{}", uid_to_name(uid));
        } else {
            let _ = pwriteln!(stdout, "{}", uid);
        }
    }

    // Print group ID
    if flag_g {
        let gid = if flag_real { get_gid() } else { get_egid() };
        if flag_name {
            let _ = pwriteln!(stdout, "{}", gid_to_name(gid));
        } else {
            let _ = pwriteln!(stdout, "{}", gid);
        }
    }

    // Print supplementary groups
    if flag_groups {
        let groups = get_groups();
        if flag_name {
            let names: Vec<String> = groups.iter().map(|g| gid_to_name(*g)).collect();
            let _ = pwriteln!(stdout, "{}", names.join(" "));
        } else {
            let nums: Vec<String> = groups.iter().map(|g| g.to_string()).collect();
            let _ = pwriteln!(stdout, "{}", nums.join(" "));
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }

    #[test]
    fn test_id_u_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-u".into()]), 0);
    }

    #[test]
    fn test_id_g_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-g".into()]), 0);
    }

    #[test]
    fn test_id_cap_g_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-G".into()]), 0);
    }

    #[test]
    fn test_id_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_id_n_without_ug_g() {
        assert_eq!(run(&mut std::io::sink(), &["-n".into()]), 1);
    }
}
