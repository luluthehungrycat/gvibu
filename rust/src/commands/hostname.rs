use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if !args.is_empty() {
        eprintln!("hostname: too many arguments");
        return 1;
    }
    let hostname = std::fs::read_to_string("/etc/hostname")
        .or_else(|_| std::fs::read_to_string("/proc/sys/kernel/hostname"))
        .unwrap_or_default();
    let hostname = hostname.trim().to_string();
    if hostname.is_empty() {
        eprintln!("hostname: cannot determine hostname");
        return 1;
    }
    writeln!(stdout, "{}", hostname).ok();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostname_rejects_args() {
        assert_eq!(run(&mut std::io::sink(), &["extra".into()]), 1);
    }
}
