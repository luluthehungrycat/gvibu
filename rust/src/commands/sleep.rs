/// sleep: delay for a specified number of seconds.
use std::io::Write;
use std::time::Duration;

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let _ = stdout;
    if args.is_empty() {
        eprintln!("sleep: usage: sleep NUMBER");
        return 2;
    }

    let seconds: i64 = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("sleep: invalid number: {}", args[0]);
            return 2;
        }
    };

    if seconds < 0 {
        eprintln!("sleep: invalid number: {}", args[0]);
        return 1;
    }

    if seconds > 0 {
        std::thread::sleep(Duration::from_secs(seconds as u64));
    }
    0
}
