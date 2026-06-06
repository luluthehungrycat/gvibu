/// uname: print system information.
use std::process::Command;

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

    // Collect each field via uname subprocess
    let mut parts = Vec::new();
    for flag in flags.chars() {
        let output = Command::new("uname")
            .arg(format!("-{}", flag))
            .output();
        match output {
            Ok(out) => {
                let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
                parts.push(val);
            }
            Err(e) => {
                eprintln!("uname: {}", e);
                return 1;
            }
        }
    }

    println!("{}", parts.join(" "));
    0
}
