/// kill: terminate a process.
use std::collections::HashMap;
use std::io::Write;

fn signal_map() -> HashMap<&'static str, i32> {
    let mut m = HashMap::new();
    m.insert("HUP", 1);
    m.insert("INT", 2);
    m.insert("QUIT", 3);
    m.insert("ILL", 4);
    m.insert("TRAP", 5);
    m.insert("ABRT", 6);
    m.insert("BUS", 7);
    m.insert("FPE", 8);
    m.insert("KILL", 9);
    m.insert("USR1", 10);
    m.insert("SEGV", 11);
    m.insert("USR2", 12);
    m.insert("PIPE", 13);
    m.insert("ALRM", 14);
    m.insert("TERM", 15);
    m.insert("STKFLT", 16);
    m.insert("CHLD", 17);
    m.insert("CONT", 18);
    m.insert("STOP", 19);
    m.insert("TSTP", 20);
    m.insert("TTIN", 21);
    m.insert("TTOU", 22);
    m.insert("URG", 23);
    m.insert("XCPU", 24);
    m.insert("XFSZ", 25);
    m.insert("VTALRM", 26);
    m.insert("PROF", 27);
    m.insert("WINCH", 28);
    m.insert("IO", 29);
    m.insert("PWR", 30);
    m.insert("SYS", 31);
    m
}

fn parse_signal(s: &str) -> Option<i32> {
    if let Ok(n) = s.parse::<i32>() {
        if (1..=31).contains(&n) {
            return Some(n);
        }
    }
    let upper = s.to_uppercase();
    signal_map().get(upper.as_str()).copied()
}

fn resolve_signal(pid_str: &str, sig_str: &str) -> Option<(i32, i32)> {
    let pid = pid_str.parse::<i32>().ok()?;
    let sig = parse_signal(sig_str)?;
    Some((pid, sig))
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("kill: missing operand");
        return 1;
    }

    // Handle -- separator: skip it, treat remaining args as positional
    let effective_args = if args[0] == "--" {
        &args[1..]
    } else {
        args
    };

    if effective_args.is_empty() {
        eprintln!("kill: missing operand");
        return 1;
    }

    // -l: list signal names
    if effective_args.len() == 1 && effective_args[0] == "-l" {
        let mut signals: Vec<_> = signal_map().into_iter().collect();
        signals.sort_by_key(|(_, n)| *n);
        for (name, num) in &signals {
            let _ = writeln!(stdout, "{:>2}) {}", num, name);
        }
        return 0;
    }

    let (pid, sig) = if effective_args[0] == "-s" {
        // kill -s SIGNAL PID
        if effective_args.len() < 3 {
            eprintln!("kill: missing operand");
            return 1;
        }
        match resolve_signal(&effective_args[2], &effective_args[1]) {
            Some(v) => v,
            None => {
                eprintln!("kill: invalid signal: {}", args[1]);
                return 1;
            }
        }
    } else if args[0].starts_with('-') && args[0].len() > 1 {
        // kill -SIGNAL PID or kill -N PID
        let sig_str = &effective_args[0][1..];
        if effective_args.len() < 2 {
            eprintln!("kill: missing operand");
            return 1;
        }
        match resolve_signal(&effective_args[1], sig_str) {
            Some(v) => v,
            None => {
                eprintln!("kill: invalid signal: {}", sig_str);
                return 1;
            }
        }
    } else {
        // kill PID (default SIGTERM)
        let pid = match effective_args[0].parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("kill: invalid pid: {}", args[0]);
                return 1;
            }
        };
        (pid, 15) // SIGTERM
    };

    // Use libc::kill via unsafe
    let result = unsafe { libc::kill(pid, sig) };
    if result == 0 {
        0
    } else {
        let err = std::io::Error::last_os_error();
        eprintln!("kill: ({}) - {}", pid, err);
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_kill_list_signals() {
        assert_eq!(run(&mut std::io::sink(), &["-l".into()]), 0);
    }

    #[test]
    fn test_kill_invalid_pid() {
        assert_eq!(run(&mut std::io::sink(), &["abc".into()]), 1);
    }

    #[test]
    fn test_parse_signal_number() {
        assert_eq!(parse_signal("9"), Some(9));
        assert_eq!(parse_signal("TERM"), Some(15));
        assert_eq!(parse_signal("KILL"), Some(9));
        assert_eq!(parse_signal("INVALID"), None);
    }
}
