/// comm: compare two sorted files line by line.
/// Output: column 1 = lines unique to file1, column 2 = lines unique to file2,
///         column 3 = lines common to both.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use crate::pwriteln;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut flag1 = true; // suppress col 1
    let mut flag2 = true; // suppress col 2
    let mut flag3 = true; // suppress col 3

    let mut files: Vec<&str> = Vec::new();
    for arg in args {
        if arg == "--" { break; }
        if arg == "-1" {
            flag1 = false;
        } else if arg == "-2" {
            flag2 = false;
        } else if arg == "-3" {
            flag3 = false;
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("comm: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg);
        }
    }

    if files.len() < 2 {
        eprintln!("comm: missing operand");
        return 1;
    }
    if files.len() > 2 {
        eprintln!("comm: extra operand: {}", files[2]);
        return 1;
    }

    let file1: Box<dyn BufRead> = if files[0] == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        match File::open(files[0]) {
            Ok(f) => Box::new(BufReader::new(f)),
            Err(e) => {
                eprintln!("comm: {}: {}", files[0], e);
                return 1;
            }
        }
    };

    let file2: Box<dyn BufRead> = if files[1] == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        match File::open(files[1]) {
            Ok(f) => Box::new(BufReader::new(f)),
            Err(e) => {
                eprintln!("comm: {}: {}", files[1], e);
                return 1;
            }
        }
    };

    let mut lines1 = file1.lines();
    let mut lines2 = file2.lines();

    let mut line1: Option<String> = None;
    let mut line2: Option<String> = None;

    loop {
        if line1.is_none() {
            match lines1.next() {
                Some(Ok(l)) => line1 = Some(l),
                Some(Err(e)) => {
                    eprintln!("comm: read error: {}", e);
                    return 1;
                }
                None => {}
            }
        }
        if line2.is_none() {
            match lines2.next() {
                Some(Ok(l)) => line2 = Some(l),
                Some(Err(e)) => {
                    eprintln!("comm: read error: {}", e);
                    return 1;
                }
                None => {}
            }
        }

        match (line1.take(), line2.take()) {
            (None, None) => break,
            (Some(l1), None) => {
                if flag1 {
                    pwriteln!(w, "{}", l1);
                }
                // keep consuming l1
                // Continue reading remaining lines1
                for line in &mut lines1 {
                    match line {
                        Ok(l) => {
                            if flag1 {
                                pwriteln!(w, "{}", l);
                            }
                        }
                        Err(_) => break,
                    }
                }
                break;
            }
            (None, Some(l2)) => {
                if flag2 {
                    pwriteln!(w, "\t{}", l2);
                }
                for line in &mut lines2 {
                    match line {
                        Ok(l) => {
                            if flag2 {
                                pwriteln!(w, "\t{}", l);
                            }
                        }
                        Err(_) => break,
                    }
                }
                break;
            }
            (Some(l1), Some(l2)) => {
                match l1.cmp(&l2) {
                    std::cmp::Ordering::Less => {
                        if flag1 {
                            pwriteln!(w, "{}", l1);
                        }
                        line2 = Some(l2); // keep l2 for next comparison
                        line1 = None;
                    }
                    std::cmp::Ordering::Greater => {
                        if flag2 {
                            pwriteln!(w, "\t{}", l2);
                        }
                        line1 = Some(l1); // keep l1 for next comparison
                        line2 = None;
                    }
                    std::cmp::Ordering::Equal => {
                        if flag3 {
                            pwriteln!(w, "\t\t{}", l1);
                        }
                        line1 = None;
                        line2 = None;
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comm_no_args() {
        let mut buf = Vec::new();
        let code = run(&mut buf, &[]);
        assert_eq!(code, 1);
    }

    #[test]
    fn test_comm_one_arg() {
        let mut buf = Vec::new();
        let code = run(&mut buf, &["/dev/null".into()]);
        assert_eq!(code, 1);
    }

    #[test]
    fn test_comm_invalid_option() {
        let mut buf = Vec::new();
        let code = run(&mut buf, &["-x".into(), "/dev/null".into(), "/dev/null".into()]);
        assert_eq!(code, 1);
    }
}
