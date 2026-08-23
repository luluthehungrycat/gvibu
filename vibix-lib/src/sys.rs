// VIBIX syscall wrappers via inline assembly.
//
// The VIBIX ABI passes the syscall number in rax and arguments in
// rdi/rsi/rdx/r8/r9. The kernel preserves only rcx and r11; every other
// general-purpose register is caller-clobbered. Keep these wrappers as the
// single unsafe boundary for the shared runtime.
#![allow(dead_code)]

pub const SYS_EXIT: u64 = 0;
pub const SYS_WRITE_COMPAT: u64 = 1;
pub const SYS_READ_COMPAT: u64 = 2;
pub const SYS_BRK: u64 = 4;
pub const SYS_WRITE_VFS: u64 = 15;
pub const SYS_READ_VFS: u64 = 14;

/// Exit the current process with the given code. Never returns.
#[inline]
pub fn sys_exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code as u64,
            clobber_abi("C"),
            options(noreturn, nostack)
        );
    }
}

/// Write `len` bytes from `buf` to file descriptor `fd`.
///
/// This is syscall 1, the compatibility path currently registered by VIBIX.
/// The raw `u64` return is intentionally decoded by `runtime`, not here.
#[inline]
pub fn sys_write(fd: i32, buf: *const u8, len: usize) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_WRITE_COMPAT => ret,
            in("rdi") fd as u64,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}

/// Read up to `len` bytes into `buf` from file descriptor `fd`.
///
/// This is syscall 2, the compatibility path currently registered by VIBIX.
#[inline]
pub fn sys_read(fd: i32, buf: *mut u8, len: usize) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_READ_COMPAT => ret,
            in("rdi") fd as u64,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}

/// Write through the canonical VIBIX VFS syscall slot.
///
/// The current kernel source does not register syscall 15 yet; callers must
/// use this only after the ABI handoff and QEMU verification.
#[inline]
pub fn sys_write_vfs(fd: i32, buf: *const u8, len: usize) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_WRITE_VFS => ret,
            in("rdi") fd as u64,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}

/// Read through the canonical VIBIX VFS syscall slot.
#[inline]
pub fn sys_read_vfs(fd: i32, buf: *mut u8, len: usize) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_READ_VFS => ret,
            in("rdi") fd as u64,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}

/// Query or update the process program break.
#[inline]
pub fn sys_brk(addr: usize) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_BRK => ret,
            in("rdi") addr as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}

/// Open a file at `path` with the given flags and mode.
#[inline]
pub fn sys_open(path: *const u8, flags: i32, mode: i32) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 12u64 => ret,
            in("rdi") path as u64,
            in("rsi") flags as u64,
            in("rdx") mode as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}

/// Close a file descriptor.
#[inline]
pub fn sys_close(fd: i32) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 13u64 => ret,
            in("rdi") fd as u64,
            clobber_abi("C"),
            options(nostack)
        );
    }
    ret
}
