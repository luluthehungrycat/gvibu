//! gvibu-vibix: bare-metal VIBIX target for the first shared GVIBU runtime slice.
//!
//! `runtime` provides allocation-free typed syscall-result conversion,
//! descriptors, checked I/O, and the allocator-facing `brk` boundary.
//! `sys` contains the raw ABI wrappers. The VIBIX command implementations
//! remain here so command flags and output semantics stay owned by GVIBU.
//!
//! # Memory layout
//! - Code loaded at `0x2000000` (flat physical addresses)
//! - Stack at `0x2002000` (grows downward, set by `_start`)
//!
//! # Syscall ABI
//! - `rax` = call number, `rdi/rsi/rdx/r8/r9` = arguments
//! - `rcx` and `r11` are preserved by the kernel; all other general registers
//!   are caller-clobbered
//! - compatibility output/input use syscall 1/2; canonical VFS 14/15 remain
//!   pending kernel registration and QEMU verification

#![cfg_attr(not(test), no_std)]

pub mod runtime;
pub mod sys;

// ── Panic handler ────────────────────────────────────────────────────────────

#[cfg(not(test))]
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

    // Print positional arguments through the checked runtime path.
    for (i, arg) in args[start..].iter().enumerate() {
        if i > 0 && runtime::write_stdout(b" ").is_err() {
            return 1;
        }
        let result = if enable_escapes {
            write_escaped_str(arg)
        } else {
            runtime::write_stdout(arg.as_bytes())
        };
        if result.is_err() {
            return 1;
        }
    }

    if newline && runtime::write_stdout(b"\n").is_err() {
        return 1;
    }

    0
}

/// Write `s` to stdout, decoding `\n`, `\t`, `\r`, `\\`, `\0NNN` escapes.
fn write_escaped_str(s: &str) -> runtime::SysResult<()> {
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
            runtime::write_stdout(&bytes[seg_start..i])?;
        }

        if i >= len {
            break;
        }

        // We're at a backslash — decode the escape
        i += 1;
        if i >= len {
            runtime::write_stdout(b"\\")?;
            break;
        }

        match bytes[i] {
            b'n' => runtime::write_stdout(b"\n")?,
            b't' => runtime::write_stdout(b"\t")?,
            b'r' => runtime::write_stdout(b"\r")?,
            b'\\' => runtime::write_stdout(b"\\")?,
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
                runtime::write_all(runtime::Fd::STDOUT, &[octal])?;
                continue;
            }
            c => {
                runtime::write_stdout(b"\\")?;
                runtime::write_all(runtime::Fd::STDOUT, &[c])?;
            }
        }
        i += 1;
    }
    Ok(())
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
