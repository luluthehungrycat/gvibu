//! VIBIX multicall binary entry point.
//!
//! The VIBIX kernel loads the flat binary at `0x2000000` and jumps to `_start`.
//! The current kernel exec path starts flat binaries with zeroed argument
//! registers. `_start` preserves and aligns the mapped user stack, then forwards
//! the kernel's argc/argv/envp registers to a bounded Rust entry point.

#![no_std]
#![no_main]

use gvibu_vibix::sys;

/// Read a bounded NUL-terminated string supplied by the kernel.
unsafe fn cstr_to_str<'a>(ptr: *const u8) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    let mut len = 0;
    while len < 4096 && *ptr.add(len) != 0 {
        len += 1;
    }
    if len == 4096 {
        return "";
    }
    core::str::from_utf8(core::slice::from_raw_parts(ptr, len)).unwrap_or("")
}

/// Count at most `limit` entries in a null-terminated kernel pointer array.
unsafe fn count_null_term_ptrs(ptr: *const *const u8, limit: usize) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let mut count = 0;
    while count < limit && !(*ptr.add(count)).is_null() {
        count += 1;
    }
    count
}

core::arch::global_asm!(
    ".section .text._start,\"ax\"",
    ".global _start",
    ".type _start,@function",
    "_start:",
    "and rsp, -16",
    "call gvibu_rust_entry",
    "ud2",
    ".size _start, .-_start",
);

// ── Entry point ──────────────────────────────────────────────────────────────

/// Rust entry point called after assembly aligns the kernel-provided stack.
#[no_mangle]
pub unsafe extern "C" fn gvibu_rust_entry(
    argc: u64,
    argv: *const *const u8,
    envp: *const *const u8,
) -> ! {
    let arg_count = if argv.is_null() { 0 } else { (argc as usize).min(256) };
    let mut arg_buf: [&str; 256] = [""; 256];
    for (index, slot) in arg_buf.iter_mut().enumerate().take(arg_count) {
        *slot = cstr_to_str(*argv.add(index));
    }
    let args = &arg_buf[..arg_count];

    let env_count = count_null_term_ptrs(envp, 128);
    let mut env_buf: [&str; 128] = [""; 128];
    for (index, slot) in env_buf.iter_mut().enumerate().take(env_count) {
        *slot = cstr_to_str(*envp.add(index));
    }

    let (cmd_name, cmd_args) = if args.is_empty() {
        ("vibix", &[][..])
    } else {
        let name = args[0].rsplit('/').next().unwrap_or(args[0]);
        (name, &args[1..])
    };
    sys::sys_exit(dispatch(cmd_name, cmd_args, &env_buf[..env_count]));
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
