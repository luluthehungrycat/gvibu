/// printenv: print all or part of environment.
use std::env;
use std::io::Write;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        let mut vars: Vec<(String, String)> = env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, value) in &vars {
            writeln!(stdout, "{}={}", key, value).ok();
        }
        return 0;
    }

    for name in args {
        writeln!(stdout, "{}", env::var(name).unwrap_or_default()).ok();
    }
    0
}
