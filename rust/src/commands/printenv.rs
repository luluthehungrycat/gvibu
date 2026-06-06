/// printenv: print all or part of environment.
use std::env;

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        let mut vars: Vec<(String, String)> = env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, value) in &vars {
            println!("{}={}", key, value);
        }
        return 0;
    }

    for name in args {
        println!("{}", env::var(name).unwrap_or_default());
    }
    0
}
