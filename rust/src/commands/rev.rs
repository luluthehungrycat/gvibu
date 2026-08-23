//! rev: reverse the characters in each input line.
use std::fs;
use std::io::{self, Read, Write};

fn reverse_lines(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for segment in input.split_inclusive('\n') {
        let (body, ending) = if let Some(body) = segment.strip_suffix('\n') {
            if let Some(body) = body.strip_suffix('\r') {
                (body, "\r\n")
            } else {
                (body, "\n")
            }
        } else {
            (segment, "")
        };
        output.extend(body.chars().rev());
        output.push_str(ending);
    }
    output
}

fn write_input(stdout: &mut dyn Write, input: &str) -> i32 {
    if stdout.write_all(reverse_lines(input).as_bytes()).is_ok() {
        0
    } else {
        1
    }
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut paths = Vec::new();
    let mut parsing_options = true;
    for arg in args {
        if parsing_options && arg == "--" {
            parsing_options = false;
        } else if parsing_options && arg.starts_with('-') {
            eprintln!("rev: invalid option: {}", arg);
            return 1;
        } else {
            paths.push(arg.as_str());
        }
    }

    if paths.is_empty() {
        let mut input = String::new();
        if io::stdin().read_to_string(&mut input).is_err() {
            eprintln!("rev: error reading standard input");
            return 1;
        }
        return write_input(stdout, &input);
    }

    let mut exit_code = 0;
    for path in paths {
        match fs::read_to_string(path) {
            Ok(input) => {
                if write_input(stdout, &input) != 0 {
                    return 1;
                }
            }
            Err(error) => {
                eprintln!("rev: {}: {}", path, error);
                exit_code = 1;
            }
        }
    }
    exit_code
}

#[cfg(test)]
mod tests {
    use super::reverse_lines;

    #[test]
    fn reverses_lines_and_preserves_endings() {
        assert_eq!(reverse_lines("abc\r\nxy"), "cba\r\nyx");
    }
}
