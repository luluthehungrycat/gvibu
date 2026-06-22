/// tail: output the last part of files.
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let mut num_lines: usize = 10;
    let mut num_bytes: Option<usize> = None;
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--" { break; }
        if arg == "-n" {
            i += 1;
            if i >= args.len() {
                eprintln!("tail: option requires an argument: -n");
                return 1;
            }
            match args[i].parse::<i64>() {
                Ok(n) if n >= 0 => num_lines = n as usize,
                Ok(_) => {
                    eprintln!("tail: invalid number of lines: 0");
                    return 1;
                }
                Err(_) => {
                    eprintln!("tail: invalid number of lines: {}", args[i]);
                    return 1;
                }
            }
        } else if arg == "-c" {
            i += 1;
            if i >= args.len() {
                eprintln!("tail: option requires an argument: -c");
                return 1;
            }
            match args[i].parse::<i64>() {
                Ok(n) if n >= 0 => num_bytes = Some(n as usize),
                Ok(_) => {
                    eprintln!("tail: invalid number of bytes: 0");
                    return 1;
                }
                Err(_) => {
                    eprintln!("tail: invalid number of bytes: {}", args[i]);
                    return 1;
                }
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            eprintln!("tail: invalid option: {}", arg);
            return 1;
        } else {
            files.push(arg.clone());
        }
        i += 1;
    }

    let mut exit_code = 0;

    if files.is_empty() || files == ["-"] {
        let stdin = io::stdin();
        let stdin = stdin.lock();
        if let Some(nbytes) = num_bytes {
            let data = read_last_bytes_reader(stdin, nbytes);
            if let Err(e) = w.write_all(&data) {
                if e.kind() == std::io::ErrorKind::BrokenPipe { return 0; }
                return 1;
            }
        } else {
            let lines = read_last_lines_reader(stdin, num_lines);
            for line in &lines {
                if let Err(e) = write!(w, "{}", line) {
                    if e.kind() == std::io::ErrorKind::BrokenPipe { return 0; }
                    return 1;
                }
            }
        }
        return 0;
    }

    for fname in &files {
        if fname == "-" {
            let stdin = io::stdin();
            let stdin = stdin.lock();
            if let Some(nbytes) = num_bytes {
                let data = read_last_bytes_reader(stdin, nbytes);
                if let Err(e) = w.write_all(&data) {
                    if e.kind() == std::io::ErrorKind::BrokenPipe { return 0; }
                    return 1;
                }
            } else {
                let lines = read_last_lines_reader(stdin, num_lines);
                for line in &lines {
                    if let Err(e) = write!(w, "{}", line) {
                        if e.kind() == std::io::ErrorKind::BrokenPipe { return 0; }
                        return 1;
                    }
                }
            }
        } else {
            match File::open(fname) {
                Ok(file) => {
                    if let Some(nbytes) = num_bytes {
                        let data = read_last_bytes_file(&file, nbytes);
                        if let Err(e) = w.write_all(&data) {
                            if e.kind() == std::io::ErrorKind::BrokenPipe { return 0; }
                            return 1;
                        }
                    } else {
                        let lines = read_last_lines_file(file, num_lines);
                        for line in &lines {
                            if let Err(e) = write!(w, "{}", line) {
                                if e.kind() == std::io::ErrorKind::BrokenPipe { return 0; }
                                return 1;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("tail: {}: {}", fname, e);
                    exit_code = 1;
                }
            }
        }
    }

    exit_code
}

fn read_last_lines_file(file: File, num_lines: usize) -> Vec<String> {
    let reader = BufReader::new(file);
    read_last_lines_reader(reader, num_lines)
}

fn read_last_lines_reader<R: BufRead>(reader: R, num_lines: usize) -> Vec<String> {
    let mut buf: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                buf.push(l + "\n");
                if buf.len() > num_lines {
                    buf.remove(0);
                }
            }
            Err(_) => break,
        }
    }
    buf
}

fn read_last_bytes_file(file: &File, num_bytes: usize) -> Vec<u8> {
    let meta = file.metadata().ok();
    let file_len = meta.map(|m| m.len()).unwrap_or(0) as usize;
    if file_len <= num_bytes {
        // Read entire file
        let mut buf = Vec::with_capacity(file_len);
        if let Ok(mut f) = file.try_clone() {
            let _ = f.read_to_end(&mut buf);
        }
        return buf;
    }
    // Seek near the end and read
    let offset = file_len - num_bytes;
    if let Ok(mut f) = file.try_clone() {
        if f.seek(SeekFrom::Start(offset as u64)).is_ok() {
            let mut buf = vec![0u8; num_bytes];
            let _ = f.read_exact(&mut buf);
            return buf;
        }
    }
    Vec::new()
}

fn read_last_bytes_reader<R: Read>(mut reader: R, num_bytes: usize) -> Vec<u8> {
    if num_bytes == 0 {
        return Vec::new();
    }
    // Ring buffer: O(num_bytes) memory regardless of input size
    let mut ring = vec![0u8; num_bytes];
    let mut pos = 0usize;
    let mut total = 0usize;
    let mut buf = [0u8; 8192];

    loop {
        let n = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        for &b in &buf[..n] {
            ring[pos] = b;
            pos = (pos + 1) % num_bytes;
            total += 1;
        }
    }

    if total <= num_bytes {
        ring[..total].to_vec()
    } else {
        // ring[pos..] then ring[..pos] gives correct byte order
        let mut result = Vec::with_capacity(num_bytes);
        result.extend_from_slice(&ring[pos..]);
        result.extend_from_slice(&ring[..pos]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_last_lines_less_than_n() {
        let lines = read_last_lines_reader("a\nb\n".as_bytes(), 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "a\n");
        assert_eq!(lines[1], "b\n");
    }

    #[test]
    fn test_read_last_lines_more_than_n() {
        let lines = read_last_lines_reader("a\nb\nc\nd\ne\n".as_bytes(), 3);
        assert_eq!(lines, vec!["c\n", "d\n", "e\n"]);
    }

    #[test]
    fn test_read_last_bytes_file_small() {
        let data = read_last_bytes_reader("hello".as_bytes(), 10);
        assert_eq!(String::from_utf8_lossy(&data), "hello");
    }

    #[test]
    fn test_read_last_bytes_large() {
        let data = read_last_bytes_reader("hello world".as_bytes(), 5);
        assert_eq!(String::from_utf8_lossy(&data), "world");
    }

    #[test]
    fn test_tail_invalid_option() {
        assert_eq!(run(&mut std::io::sink(), &["-x".into()]), 1);
    }

    #[test]
    fn test_tail_dev_null() {
        assert_eq!(run(&mut std::io::sink(), &["/dev/null".into()]), 0);
    }

    #[test]
    fn test_tail_c_missing_arg() {
        assert_eq!(run(&mut std::io::sink(), &["-c".into()]), 1);
    }
}
