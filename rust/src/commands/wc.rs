/// wc: print newline, word, and byte counts.
use std::fs;
use std::io::{self, Read};

fn count_data(data: &str) -> (usize, usize, usize) {
    let lines = data.matches('\n').count();
    let words = if data.is_empty() {
        0
    } else {
        data.split_whitespace().count()
    };
    let bytes = data.len();
    (lines, words, bytes)
}

pub fn run(args: &[String]) -> i32 {
    let mut flag_l = false;
    let mut flag_w = false;
    let mut flag_c = false;

    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'l' => flag_l = true,
                    'w' => flag_w = true,
                    'c' => flag_c = true,
                    _ => {
                        eprintln!("wc: invalid option: -{}", ch);
                        return 1;
                    }
                }
            }
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    // Default: all three
    if !flag_l && !flag_w && !flag_c {
        flag_l = true;
        flag_w = true;
        flag_c = true;
    }

    let fmt = |lines: usize, words: usize, bytes: usize, name: &str| -> String {
        let mut parts = Vec::new();
        if flag_l {
            parts.push(format!("{:>7}", lines));
        }
        if flag_w {
            parts.push(format!("{:>7}", words));
        }
        if flag_c {
            parts.push(format!("{:>7}", bytes));
        }
        if !name.is_empty() {
            parts.push(name.to_string());
        }
        parts.join(" ")
    };

    let mut exit_code = 0;
    let mut total_l = 0usize;
    let mut total_w = 0usize;
    let mut total_c = 0usize;

    if files.is_empty() {
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buf) {
            eprintln!("wc: stdin: {}", e);
            return 1;
        }
        let (l, w, c) = count_data(&buf);
        println!("{}", fmt(l, w, c, ""));
        return 0;
    }

    for fname in &files {
        let data = match fs::read_to_string(fname) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("wc: {}: {}", fname, e);
                exit_code = 1;
                continue;
            }
        };

        let (l, w, c) = count_data(&data);
        total_l += l;
        total_w += w;
        total_c += c;
        println!("{}", fmt(l, w, c, fname));
    }

    if files.len() > 1 {
        println!("{}", fmt(total_l, total_w, total_c, "total"));
    }

    exit_code
}
