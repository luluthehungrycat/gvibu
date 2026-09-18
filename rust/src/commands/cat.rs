use crate::pwrite;
/// cat: concatenate files and print to stdout.
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut number_lines = false;
    let mut files: Vec<&str> = Vec::new();

    for arg in args {
        if arg == "--" {
            break;
        }
        if arg == "-n" {
            number_lines = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("cat: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.as_str());
        }
    }

    let mut line_num: usize = 1;
    let mut exit_code = 0;

    if files.is_empty() {
        let stdin = io::stdin();
        return print_lines(w, stdin.lock(), number_lines, &mut line_num, "-");
    }

    for filename in files {
        if filename == "-" {
            let stdin = io::stdin();
            exit_code |= print_lines(w, stdin.lock(), number_lines, &mut line_num, "-");
        } else {
            match File::open(filename) {
                Ok(file) => {
                    exit_code |= print_lines(
                        w,
                        BufReader::new(file),
                        number_lines,
                        &mut line_num,
                        filename,
                    );
                }
                Err(e) => {
                    eprintln!("cat: {}: {}", filename, e);
                    exit_code = 1;
                }
            }
        }
    }

    exit_code
}

fn print_lines<R: Read>(
    w: &mut dyn Write,
    mut reader: R,
    number: bool,
    line_num: &mut usize,
    src: &str,
) -> i32 {
    if !number {
        return match io::copy(&mut reader, w) {
            Ok(_) => 0,
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => 0,
            Err(e) => {
                eprintln!("cat: {}: write error: {}", src, e);
                1
            }
        };
    }

    let mut contents = Vec::new();
    if let Err(e) = reader.read_to_end(&mut contents) {
        eprintln!("cat: {}: read error: {}", src, e);
        return 1;
    }

    for (index, byte) in contents.iter().enumerate() {
        if number && (index == 0 || contents[index - 1] == b'\n') {
            pwrite!(w, "{:>6}\t", line_num);
            *line_num += 1;
        }
        if let Err(e) = w.write_all(std::slice::from_ref(byte)) {
            if e.kind() == io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_cat_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 0);
    }

    #[test]
    fn test_cat_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_cat_n_flag_dev_null() {
        assert_eq!(
            run(&mut std::io::sink(), &["-n".into(), "/dev/null".into()]),
            0
        );
    }

    #[test]
    fn test_cat_preserves_unterminated_line() {
        let mut output = Vec::new();
        let input = std::io::Cursor::new(b"a".to_vec());
        assert_eq!(print_lines(&mut output, input, false, &mut 1, "-"), 0);
        assert_eq!(output, b"a");
    }
}
