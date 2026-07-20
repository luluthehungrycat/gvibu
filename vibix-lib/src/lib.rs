//! gvibu-vibix: bare-metal VIBIX target for 58 Unix coreutils.
//!
//! Provides syscall wrappers, a no-alloc `VibixWriter` implementing
//! `core::fmt::Write`, and no_std implementations of the 5 simplest
//! commands (true, false, echo, yes, printenv).
//!
//! # Memory layout
//! - Code loaded at `0x2000000` (flat physical addresses)
//! - Stack at `0x2002000` (grows downward, set by `_start`)
//!
//! # Syscall ABI
//! - `rax` = call number, `rdi/rsi/rdx` = args
//! - Return in `rax`. rcx, r11, and arg regs clobbered.
//! - `0` = exit, `1` = write, `2` = read, `11` = open, `12` = close

#![no_std]

pub mod sys;

// ── Panic handler ────────────────────────────────────────────────────────────

#[panic_handler]
fn lib_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    // PanicInfo::message() returns PanicMessage which implements Display
    let _ = core::write!(VibixStderr, "VIBIX PANIC: {}\n", info.message());
    sys::sys_exit(255);
}

use core::fmt::Write;

// ── Writer ───────────────────────────────────────────────────────────────────

/// Writes to VIBIX stdout (fd 1) via `sys_write`.
/// Implements `core::fmt::Write` so it can be used with `write!`/`writeln!`.
pub struct VibixWriter;

impl Write for VibixWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sys::sys_write(1, s.as_ptr(), s.len());
        Ok(())
    }
}

/// Writes to VIBIX stderr (fd 2) via `sys_write`.
pub struct VibixStderr;

impl Write for VibixStderr {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sys::sys_write(2, s.as_ptr(), s.len());
        Ok(())
    }
}

// ── Raw I/O helpers ──────────────────────────────────────────────────────────

/// Write a string slice to stdout directly (no fmt overhead).
pub fn vibix_print(s: &str) {
    sys::sys_write(1, s.as_ptr(), s.len());
}

/// Write a string slice to stderr directly.
pub fn vibix_eprint(s: &str) {
    sys::sys_write(2, s.as_ptr(), s.len());
}

/// Write a single byte to stdout.
pub fn vibix_putchar(c: u8) {
    sys::sys_write(1, &c as *const u8, 1);
}

// ── Command implementations ──────────────────────────────────────────────────

/// `true` — exit with code 0.
pub fn cmd_true(_args: &[&str]) -> i32 {
    0
}

/// `false` — exit with code 1.
pub fn cmd_false(_args: &[&str]) -> i32 {
    1
}

/// `echo` — print arguments joined by spaces, then a newline.
///
/// Supported flags (consumed before first positional arg):
///   `-n`  omit trailing newline
///   `-e`  interpret `\n`, `\t`, `\r`, `\\`, `\0NNN` escapes
///   `-E`  disable escape interpretation (default)
///   `--`  end of option processing
pub fn cmd_echo(args: &[&str]) -> i32 {
    let mut newline = true;
    let mut enable_escapes = false;
    let mut start = 0;

    // Parse option flags
    while start < args.len() {
        let arg = args[start];
        if arg == "--" {
            start += 1;
            break;
        }
        if arg == "-n" {
            newline = false;
            start += 1;
        } else if arg == "-e" {
            enable_escapes = true;
            start += 1;
        } else if arg == "-E" {
            enable_escapes = false;
            start += 1;
        } else {
            break;
        }
    }

    // Print positional arguments
    for (i, arg) in args[start..].iter().enumerate() {
        if i > 0 {
            vibix_print(" ");
        }
        if enable_escapes {
            write_escaped_str(arg);
        } else {
            vibix_print(arg);
        }
    }

    if newline {
        vibix_print("\n");
    }

    0
}

/// Write `s` to stdout, decoding `\n`, `\t`, `\r`, `\\`, `\0NNN` escapes.
fn write_escaped_str(s: &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // Find next escape or end
        let seg_start = i;
        while i < len && bytes[i] != b'\\' {
            i += 1;
        }

        // Write literal segment
        if i > seg_start {
            if let Ok(seg) = core::str::from_utf8(&bytes[seg_start..i]) {
                vibix_print(seg);
            }
        }

        if i >= len {
            break;
        }

        // We're at a backslash — decode the escape
        i += 1; // skip '\'
        if i >= len {
            vibix_putchar(b'\\');
            break;
        }

        match bytes[i] {
            b'n' => vibix_putchar(b'\n'),
            b't' => vibix_putchar(b'\t'),
            b'r' => vibix_putchar(b'\r'),
            b'\\' => vibix_putchar(b'\\'),
            b'0' => {
                i += 1;
                let mut octal: u8 = 0;
                for _ in 0..3 {
                    if i < len && bytes[i] >= b'0' && bytes[i] <= b'7' {
                        octal = octal.wrapping_mul(8).wrapping_add(bytes[i] - b'0');
                        i += 1;
                    } else {
                        break;
                    }
                }
                // i is already past the octal digits; the loop increment will skip
                // one extra, so we decrement by 1 to compensate for the +=1 below
                // Actually, let me restructure:
                // We've already consumed the '0' at i-1 and the octal digits.
                // The outer loop will do i += 1 below. So we need i to point to
                // the last consumed digit + 1 already after the loop, then the
                // i += 1 at the bottom will be one too many.
                // Let's handle this with continue to skip the bottom i += 1.
                vibix_putchar(octal);
                continue; // skip the i += 1 below since we advanced i manually
            }
            c => {
                vibix_putchar(b'\\');
                vibix_putchar(c);
            }
        }
        i += 1;
    }
}

/// `yes` — repeatedly output a string (default "y") until killed.
pub fn cmd_yes(args: &[&str]) -> i32 {
    loop {
        if args.is_empty() {
            vibix_print("y\n");
        } else {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    vibix_print(" ");
                }
                vibix_print(arg);
            }
            vibix_print("\n");
        }
    }
}

/// `printenv` — print environment variables (if available) or nothing.
///
/// In VIBIX, environment variables come from the kernel via envp.
/// If `envp` is `None`, no variables are shown.
pub fn cmd_printenv(args: &[&str], envp: Option<&[&str]>) -> i32 {
    if args.is_empty() {
        // Print all environment variables
        if let Some(vars) = envp {
            for var in vars {
                vibix_print(var);
                vibix_print("\n");
            }
        }
        0
    } else {
        // Print specific variables
        if let Some(vars) = envp {
            for name in args {
                let mut found = false;
                for var in vars {
                    if let Some(eq_pos) = var.find('=') {
                        if &var[..eq_pos] == *name {
                            vibix_print(&var[eq_pos + 1..]);
                            vibix_print("\n");
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    vibix_print("\n");
                }
            }
        } else {
            for _ in args {
                vibix_print("\n");
            }
        }
        0
    }
}
