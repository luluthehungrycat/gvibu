pub mod commands;

use std::sync::OnceLock;

/// One-shot WASM stdin buffer. The browser runner sets it before dispatch
/// and reads it from inside commands that previously panicked on `io::stdin()`.
/// Cleared by `wasm-lib` after dispatch; native builds never touch it.
static STDIN: OnceLock<String> = OnceLock::new();

/// Set the WASM stdin buffer. Subsequent `get_wasm_stdin` calls return this
/// value. Only the first call wins; later calls are silently ignored.
pub fn set_wasm_stdin(s: &str) {
    let _ = STDIN.set(s.to_string());
}

/// Borrow the WASM stdin buffer if one was set. Returns `None` on native
/// builds or after a dispatch that did not provide stdin.
pub fn get_wasm_stdin() -> Option<&'static str> {
    STDIN.get().map(|s| s.as_str())
}

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
