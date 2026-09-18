use crate::pwriteln;
/// yes: output a string repeatedly.
use std::io::Write;

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    let text = if args.is_empty() {
        "y".to_string()
    } else {
        args.join(" ")
    };

    loop {
        pwriteln!(w, "{}", text);
        if let Err(e) = w.flush() {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    struct FlushTrackingWriter {
        bytes_written: usize,
        flushes: usize,
    }

    impl Write for FlushTrackingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.bytes_written >= 2 {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed"));
            }
            let count = bytes.len().min(2 - self.bytes_written);
            self.bytes_written += count;
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            Ok(())
        }
    }

    #[test]
    fn test_yes_flushes_each_line() {
        let mut output = FlushTrackingWriter {
            bytes_written: 0,
            flushes: 0,
        };
        assert_eq!(run(&mut output, &[]), 0);
        assert_eq!(output.flushes, 1);
    }
}
