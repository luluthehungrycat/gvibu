pub fn run(args: &[String]) -> i32 {
    if !args.is_empty() {
        eprintln!("whoami: too many arguments");
        return 1;
    }

    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_default();

    if user.is_empty() {
        eprintln!("whoami: cannot find username");
        return 1;
    }

    println!("{}", user);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whoami_rejects_args() {
        assert_eq!(run(&["extra".into()]), 1);
    }

    #[test]
    fn test_whoami_with_env() {
        // Set USER and verify it's used
        std::env::set_var("USER", "testuser");
        assert_eq!(run(&[]), 0);
    }
}
