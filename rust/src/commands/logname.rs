use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if !args.is_empty() {
        eprintln!("logname: too many arguments");
        return 1;
    }
    let user = std::env::var("LOGNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_default();
    if user.is_empty() {
        eprintln!("logname: no login name");
        return 1;
    }
    writeln!(stdout, "{}", user).ok();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logname_rejects_args() {
        assert_eq!(run(&mut std::io::sink(), &["extra".into()]), 1);
    }

    #[test]
    fn test_logname_with_env() {
        std::env::set_var("LOGNAME", "testuser");
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }
}
