//! VIBIX multicall binary entry point.
//!
//! The VIBIX kernel loads the flat binary at `0x2000000` and jumps to `_start`.
//! Register state on entry:
//!   - `rdi` = argc (number of arguments)
//!   - `rsi` = argv (pointer to array of null-terminated strings)
//!   - `rdx` = envp (pointer to null-terminated array of "KEY=VALUE" strings)
//!
//! Stack pointer must be set to `0x2006000` (grows downward, 8 KiB available).
//! This is done via an inline `mov rsp, 0x2002000` as the very first instruction.
//! _start is declared with no arguments because the compiler prologue would use
//! whatever garbage RSP the kernel left; we read rdi/rsi/rdx directly via asm
//! after setting RSP to the VIBIX stack.

#![no_std]
#![no_main]

use gvibu_vibix::sys;

// ── C-string helper ──────────────────────────────────────────────────────────

/// Convert a null-terminated C string pointer to a `&str`.
unsafe fn cstr_to_str<'a>(ptr: *const u8) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    core::str::from_utf8(core::slice::from_raw_parts(ptr, len)).unwrap_or("")
}

/// Walk a null-terminated pointer array to count entries.
unsafe fn count_null_term_ptrs(mut ptr: *const *const u8) -> usize {
    let mut count = 0;
    while !(*ptr).is_null() {
        count += 1;
        ptr = ptr.add(1);
    }
    count
}

// ── Entry point ──────────────────────────────────────────────────────────────

/// VIBIX flat binary entry point.
///
/// # Safety
/// Called directly by the VIBIX kernel with raw pointers in rdi/rsi/rdx.
/// We must set RSP to 0x2002000 before any operation that touches the stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Read kernel-provided registers and set VIBIX stack.
    // The asm block reads rdi/rsi/rdx (argc/argv/envp) and sets RSP,
    // all before the compiler can emit any prologue that touches the old stack.
    let argc: u64;
    let argv: *const *const u8;
    let envp: *const *const u8;
    core::arch::asm!(
        "mov rsp, 0x2006000",
        "mov {}, rdi",
        "mov {}, rsi",
        "mov {}, rdx",
        out(reg) argc,
        out(reg) argv,
        out(reg) envp,
        options(nostack)
    );

    vibix_main(argc, argv, envp)
}

/// Real entry point, called after RSP is on the VIBIX stack.
unsafe fn vibix_main(argc: u64, argv: *const *const u8, envp: *const *const u8) -> ! {
    // Build &[&str] from raw args using a stack-allocated buffer.
    // Max 256 args (more than enough for any VIBIX program).
    let arg_count = (argc as usize).min(256);
    let mut arg_buf: [&str; 256] = [""; 256];

    if arg_count > 0 && !argv.is_null() {
        for i in 0..arg_count {
            arg_buf[i] = cstr_to_str((*argv.add(i)) as *const u8);
        }
    }
    let args: &[&str] = &arg_buf[..arg_count];

    // Build &[&str] from envp
    let env_count = if !envp.is_null() {
        count_null_term_ptrs(envp).min(128)
    } else {
        0
    };
    let mut env_buf: [&str; 128] = [""; 128];
    for i in 0..env_count {
        env_buf[i] = cstr_to_str((*envp.add(i)) as *const u8);
    }
    let env_vars: &[&str] = &env_buf[..env_count];

    // Determine command name from argv[0]
    let cmd_name = if !args.is_empty() { args[0] } else { "" };
    let cmd_args = if args.len() > 1 { &args[1..] } else { &[] };

    // Dispatch
    let exit_code = dispatch(cmd_name, cmd_args, env_vars);

    sys::sys_exit(exit_code);
}

// ── Command dispatch ─────────────────────────────────────────────────────────

/// Route the command name to the appropriate implementation.
fn dispatch(cmd: &str, args: &[&str], env_vars: &[&str]) -> i32 {
    match cmd {
        "true" => gvibu_vibix::cmd_true(args),
        "false" => gvibu_vibix::cmd_false(args),
        "echo" => gvibu_vibix::cmd_echo(args),
        "yes" => gvibu_vibix::cmd_yes(args),
        "printenv" => gvibu_vibix::cmd_printenv(args, Some(env_vars)),

        // Multicall binary name — default behavior
        "gvibu" | "vibix" => {
            if args.is_empty() {
                gvibu_vibix::vibix_eprint("vibix: try true, false, echo, yes, printenv\n");
                1
            } else {
                dispatch(args[0], &args[1..], env_vars)
            }
        }

        // Unknown command
        _ => {
            gvibu_vibix::vibix_eprint("vibix: ");
            gvibu_vibix::vibix_eprint(cmd);
            gvibu_vibix::vibix_eprint(": command not found\n");
            1
        }
    }
}
