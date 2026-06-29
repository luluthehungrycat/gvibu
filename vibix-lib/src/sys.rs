// VIBIX syscall wrappers via inline assembly.
//
// Syscall ABI:
//   rax = syscall#, rdi/rsi/rdx = args, return in rax
//   rax, rcx, r11, rdi, rsi, rdx, r8, r9, r10 clobbered
//
// VIBIX syscalls (all physical addresses, flat memory model):
//   0 = exit(int code)        — never returns
//   1 = write(fd, buf, len)   — returns bytes written
//   2 = read(fd, buf, len)    — returns bytes read
//  11 = open(path, flags)     — returns fd
//  12 = close(fd)             — returns 0
#![allow(dead_code)]

/// Exit the current process with the given code. Never returns.
#[inline]
pub fn sys_exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 0u64,
            in("rdi") code as u64,
            options(noreturn, nostack)
        );
    }
}

/// Write `len` bytes from `buf` to file descriptor `fd`.
/// Returns the number of bytes written, or a negative error code.
#[inline]
pub fn sys_write(fd: i32, buf: *const u8, len: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1u64,
            in("rdi") fd as u64,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Read up to `len` bytes into `buf` from file descriptor `fd`.
/// Returns the number of bytes read, 0 on EOF, or a negative error code.
#[inline]
pub fn sys_read(fd: i32, buf: *mut u8, len: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 2u64,
            in("rdi") fd as u64,
            in("rsi") buf as u64,
            in("rdx") len as u64,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Open a file at `path` with the given `flags` and `mode`.
/// Returns a file descriptor, or a negative error code.
#[inline]
pub fn sys_open(path: *const u8, flags: i32, mode: i32) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 11u64,
            in("rdi") path as u64,
            in("rsi") flags as u64,
            in("rdx") mode as u64,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// Close a file descriptor.
/// Returns 0 on success, or a negative error code.
#[inline]
pub fn sys_close(fd: i32) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 12u64,
            in("rdi") fd as u64,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
    }
    ret
}
