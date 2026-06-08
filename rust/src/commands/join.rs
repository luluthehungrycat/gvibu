/// join: join lines of two files on a common field.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut field: usize = 1;
    let mut files: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { i += 1; break; }
        if arg == "-j" {
            i += 1;
            if i >= args.len() {
                eprintln!("join: option requires an argument: -j");
                return 1;
            }
            match args[i].parse::<usize>() {
                Ok(n) if n > 0 => field = n,
                _ => {
                    eprintln!("join: invalid field number: {}", args[i]);
                    return 1;
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("join: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    if files.len() < 2 {
        eprintln!("join: missing operand");
        return 1;
    }

    let file1_path = &files[0];
    let file2_path = &files[1];

    let lines1 = match read_lines(file1_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("join: {}: {}", file1_path, e);
            return 1;
        }
    };
    let lines2 = match read_lines(file2_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("join: {}: {}", file2_path, e);
            return 1;
        }
    };

    let field_idx = field - 1;

    // Build a map from key to lines in file2
    let mut map2: std::collections::HashMap<String, Vec<Vec<String>>> = std::collections::HashMap::new();
    for line in &lines2 {
        let fields: Vec<String> = line.split_whitespace().map(String::from).collect();
        if field_idx < fields.len() {
            map2.entry(fields[field_idx].clone())
                .or_default()
                .push(fields);
        }
    }

    // Join file1 lines with file2
    for line in &lines1 {
        let fields: Vec<String> = line.split_whitespace().map(String::from).collect();
        if field_idx >= fields.len() {
            continue;
        }
        let key = &fields[field_idx];
        if let Some(matches) = map2.get(key) {
            for m in matches {
                // Print join field + all file1 fields + all file2 fields
                pwrite!(w, "{}", key);
                for (j, f) in fields.iter().enumerate() {
                    if j != field_idx {
                        pwrite!(w, " {}", f);
                    }
                }
                for (j, f) in m.iter().enumerate() {
                    if j != field_idx {
                        pwrite!(w, " {}", f);
                    }
                }
                pwriteln!(w);
            }
        }
    }

    0
}

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::sink;

    #[test]
    fn test_join_no_args() {
        assert_eq!(run(&mut sink(), &[]), 1);
    }

    #[test]
    fn test_join_one_arg() {
        assert_eq!(run(&mut sink(), &["file1"]), 1);
    }
}
