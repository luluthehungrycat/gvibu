//! expand: convert tabs to spaces.
use std::fs;
use std::io::{self, Read, Write};

fn parse_tab_stops(value: &str) -> Result<Vec<usize>, String> {
    let mut stops = Vec::new();
    for part in value.split(',') {
        let stop = part
            .parse::<usize>()
            .map_err(|_| format!("invalid tab stop: {}", part))?;
        if stop == 0 || stops.last().is_some_and(|last| stop <= *last) {
            return Err(format!("invalid tab stop: {}", part));
        }
        stops.push(stop);
    }
    if stops.is_empty() {
        return Err("empty tab stop list".to_string());
    }
    Ok(stops)
}

fn spaces_to_next_tab(column: usize, stops: Option<&[usize]>) -> usize {
    match stops {
        None => 8 - (column % 8),
        Some(stops) => {
            if let Some(next) = stops.iter().copied().find(|stop| *stop > column) {
                return next - column;
            }
            let interval = if stops.len() >= 2 {
                stops[stops.len() - 1] - stops[stops.len() - 2]
            } else {
                stops[0]
            };
            let interval = interval.max(1);
            let offset = column - stops[stops.len() - 1];
            interval - (offset % interval)
        }
    }
}

fn expand_content(input: &str, stops: Option<&[usize]>, initial_only: bool) -> String {
    let mut output = String::with_capacity(input.len());
    let mut column = 0;
    let mut at_line_start = true;

    for ch in input.chars() {
        match ch {
            '\n' => {
                output.push(ch);
                column = 0;
                at_line_start = true;
            }
            '\t' if !initial_only || at_line_start => {
                let spaces = spaces_to_next_tab(column, stops);
                output.extend(std::iter::repeat(' ').take(spaces));
                column += spaces;
            }
            '\t' => output.push(ch),
            _ => {
                output.push(ch);
                column += 1;
                at_line_start = false;
            }
        }
    }
    output
}

fn write_content(
    stdout: &mut dyn Write,
    input: &str,
    stops: Option<&[usize]>,
    initial_only: bool,
) -> i32 {
    let output = expand_content(input, stops, initial_only);
    if stdout.write_all(output.as_bytes()).is_ok() {
        0
    } else {
        1
    }
}

pub fn run(stdout: &mut dyn Write, args: &[String]) -> i32 {
    let mut stops = None;
    let mut initial_only = false;
    let mut paths = Vec::new();
    let mut i = 0;
    let mut parsing_options = true;

    while i < args.len() {
        let arg = &args[i];
        if parsing_options && arg == "--" {
            parsing_options = false;
        } else if parsing_options && arg == "-i" {
            initial_only = true;
        } else if parsing_options && arg == "-t" {
            i += 1;
            if i >= args.len() {
                eprintln!("expand: option requires an argument: -t");
                return 1;
            }
            match parse_tab_stops(&args[i]) {
                Ok(parsed) => stops = Some(parsed),
                Err(error) => {
                    eprintln!("expand: {}", error);
                    return 1;
                }
            }
        } else if parsing_options && arg.starts_with("-t") && arg.len() > 2 {
            match parse_tab_stops(&arg[2..]) {
                Ok(parsed) => stops = Some(parsed),
                Err(error) => {
                    eprintln!("expand: {}", error);
                    return 1;
                }
            }
        } else if parsing_options && arg.starts_with('-') {
            eprintln!("expand: invalid option: {}", arg);
            return 1;
        } else {
            paths.push(arg.as_str());
        }
        i += 1;
    }

    if paths.is_empty() {
        let mut input = String::new();
        if io::stdin().read_to_string(&mut input).is_err() {
            eprintln!("expand: error reading standard input");
            return 1;
        }
        return write_content(stdout, &input, stops.as_deref(), initial_only);
    }

    let mut exit_code = 0;
    for path in paths {
        match fs::read_to_string(path) {
            Ok(input) => {
                if write_content(stdout, &input, stops.as_deref(), initial_only) != 0 {
                    return 1;
                }
            }
            Err(error) => {
                eprintln!("expand: {}: {}", path, error);
                exit_code = 1;
            }
        }
    }
    exit_code
}

#[cfg(test)]
mod tests {
    use super::{expand_content, parse_tab_stops};

    #[test]
    fn expands_default_tabs() {
        assert_eq!(expand_content("\tab\n", None, false), "        ab\n");
    }

    #[test]
    fn parses_custom_tab_stops() {
        let stops = parse_tab_stops("4,8").unwrap();
        assert_eq!(expand_content("ab\tcd\n", Some(&stops), false), "ab  cd\n");
    }
}
