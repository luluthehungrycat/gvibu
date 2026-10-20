//! VIBIX syscall wrappers.
//!
//! Process read/write/exit use syscall numbers 2/1/0.  File open/close are
//! VFS calls 12/13; 11 is mmap in the kernel.  The assembly boundary saves
//! every SysV callee-saved register because VIBIX syscall handlers do not
//! promise to preserve them.

core::arch::global_asm!(
    ".section .text.gvibu_syscalls,\"ax\"",
    ".macro SAVE_CALLEE_SAVED",
    "push rbx",
    "push rbp",
    "push r12",
    "push r13",
    "push r14",
    "push r15",
    ".endm",
    ".macro RESTORE_CALLEE_SAVED",
    "pop r15",
    "pop r14",
    "pop r13",
    "pop r12",
    "pop rbp",
    "pop rbx",
    ".endm",
    ".global gvibu_sys_exit",
    ".type gvibu_sys_exit,@function",
    "gvibu_sys_exit:",
    "SAVE_CALLEE_SAVED",
    "mov rax, 0",
    "syscall",
    "RESTORE_CALLEE_SAVED",
    "ret",
    ".size gvibu_sys_exit, .-gvibu_sys_exit",
    ".global gvibu_sys_write",
    ".type gvibu_sys_write,@function",
    "gvibu_sys_write:",
    "SAVE_CALLEE_SAVED",
    "mov rax, 1",
    "syscall",
    "RESTORE_CALLEE_SAVED",
    "ret",
    ".size gvibu_sys_write, .-gvibu_sys_write",
    ".global gvibu_sys_read",
    ".type gvibu_sys_read,@function",
    "gvibu_sys_read:",
    "SAVE_CALLEE_SAVED",
    "mov rax, 2",
    "syscall",
    "RESTORE_CALLEE_SAVED",
    "ret",
    ".size gvibu_sys_read, .-gvibu_sys_read",
    ".global gvibu_sys_open",
    ".type gvibu_sys_open,@function",
    "gvibu_sys_open:",
    "SAVE_CALLEE_SAVED",
    "mov rax, 12",
    "syscall",
    "RESTORE_CALLEE_SAVED",
    "ret",
    ".size gvibu_sys_open, .-gvibu_sys_open",
    ".global gvibu_sys_close",
    ".type gvibu_sys_close,@function",
    "gvibu_sys_close:",
    "SAVE_CALLEE_SAVED",
    "mov rax, 13",
    "syscall",
    "RESTORE_CALLEE_SAVED",
    "ret",
    ".size gvibu_sys_close, .-gvibu_sys_close",
    ".global gvibu_sys_brk",
    ".type gvibu_sys_brk,@function",
    "gvibu_sys_brk:",
    "SAVE_CALLEE_SAVED",
    "mov rax, 4",
    "syscall",
    "RESTORE_CALLEE_SAVED",
    "ret",
    ".size gvibu_sys_brk, .-gvibu_sys_brk",
);

unsafe extern "C" {
    fn gvibu_sys_exit(code: i32) -> i32;
    fn gvibu_sys_write(fd: i32, buffer: *const u8, length: usize) -> u64;
    fn gvibu_sys_read(fd: i32, buffer: *mut u8, length: usize) -> u64;
    fn gvibu_sys_open(path: *const u8, flags: i32, mode: i32) -> u64;
    fn gvibu_sys_close(fd: i32) -> u64;
    fn gvibu_sys_brk(addr: usize) -> u64;
}

/// Exit the current process with `code`. The kernel should not return.
#[inline]
pub fn sys_exit(code: i32) -> ! {
    unsafe { gvibu_sys_exit(code) };
    loop {
        core::hint::spin_loop();
    }
}

/// Write bytes to a process file descriptor.
#[inline]
/// # Safety
/// `buf` must be readable for `len` bytes in the current VIBIX address space.
pub unsafe fn sys_write(fd: i32, buf: *const u8, len: usize) -> u64 {
    unsafe { gvibu_sys_write(fd, buf, len) }
}

/// Read bytes from a process file descriptor.
#[inline]
/// # Safety
/// `buf` must be writable for `len` bytes in the current VIBIX address space.
pub unsafe fn sys_read(fd: i32, buf: *mut u8, len: usize) -> u64 {
    unsafe { gvibu_sys_read(fd, buf, len) }
}

/// Open a VFS path using the kernel's syscall 12 ABI.
#[inline]
/// # Safety
/// `path` must point to a readable, NUL-terminated path string.
pub unsafe fn sys_open(path: *const u8, flags: i32, mode: i32) -> u64 {
    unsafe { gvibu_sys_open(path, flags, mode) }
}

/// Close a VFS file descriptor using syscall 13.
#[inline]
pub fn sys_close(fd: i32) -> u64 {
    unsafe { gvibu_sys_close(fd) }
}

/// Query or update the kernel-managed process break using syscall 4.
#[inline]
pub fn sys_brk(addr: usize) -> u64 {
    unsafe { gvibu_sys_brk(addr) }
}
