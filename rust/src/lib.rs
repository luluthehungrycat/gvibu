pub mod commands;

/// Write a line to output, returning 0 on BrokenPipe (clean exit) and 1 on other errors.
#[macro_export]
macro_rules! pwriteln {
    ($dst:expr) => {
        if let Err(e) = writeln!($dst) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    };
    ($dst:expr, $($rest:tt)*) => {
        if let Err(e) = writeln!($dst, $($rest)*) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    };
}

/// Write to output, returning 0 on BrokenPipe (clean exit) and 1 on other errors.
#[macro_export]
macro_rules! pwrite {
    ($dst:expr, $($rest:tt)*) => {
        if let Err(e) = write!($dst, $($rest)*) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return 0;
            }
            return 1;
        }
    };
}
