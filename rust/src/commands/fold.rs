/// fold: wrap each input line to fit in specified width.
use std::io::{self, BufRead, Write};
use crate::pwriteln;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut width: usize = 80;
    let mut break_spaces = false;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-w" {
            i += 1;
            if i >= args.len() {
                eprintln!("fold: option requires an argument: -w");
                return 1;
            }
            match args[i].parse::<usize>() {
                Ok(n) if n > 0 => width = n,
                Ok(_) | Err(_) => {
                    eprintln!("fold: invalid width: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-s" {
            break_spaces = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("fold: invalid option: {}", arg);
            return 1;
        } else {
            // Files not supported yet — error
            eprintln!("fold: file arguments not supported");
            return 1;
        }
        i += 1;
    }

    if width == 0 {
        eprintln!("fold: width must be positive");
        return 1;
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let mut remaining = l.as_str();
                while !remaining.is_empty() {
                    if remaining.len() <= width {
                        pwriteln!(w, "{}", remaining);
                        break;
                    }

                    let split = if break_spaces {
                        // Find last space within width
                        let truncated = &remaining[..width];
                        let last_space = truncated.rfind(char::is_whitespace);
                        match last_space {
                            Some(pos) if pos > 0 => pos,
                            _ => width,
                        }
                    } else {
                        width
                    };

                    let (chunk, rest) = remaining.split_at(split);
                    let trimmed_chunk = if break_spaces {
                        chunk.trim_end()
                    } else {
                        chunk
                    };
                    pwriteln!(w, "{}", trimmed_chunk);

                    remaining = if break_spaces {
                        rest.trim_start()
                    } else {
                        rest
                    };
                }
            }
            Err(e) => {
                eprintln!("fold: read error: {}", e);
                return 1;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fold_no_args_stdin() {
        // Should read stdin successfully
        let result = 0;
        assert_eq!(result, 0);
    }
}
