;==============================================================================
; user_echo_init.asm — Echo init for VIBIX (PID 1 flat binary)
;
; Port of the GVIBU echo command to run as the init process on VIBIX.
;
; Assemble:   nasm -f bin kernel/user_echo_init.asm -o user_echo_init.bin
; Embed in:   kernel_rust/src/process.rs (replace include_bytes path)
; Build/run:  make run  (QEMU serial output via make in VIBIX repo root)
;
; Reference:  gvibu-ai-lab/rust/src/commands/echo_cmd.rs
;
; ── Syscall ABI ──────────────────────────────────────────────────────────────
;   rax = syscall number
;   rdi = arg1, rsi = arg2, rdx = arg3, r8 = arg4, r9 = arg5
;   Return value in rax.
;   ALL registers except rcx, r11 are CLOBBERED by syscall.
;
; ── Syscalls ─────────────────────────────────────────────────────────────────
;   0 = exit(int code)                       → never returns
;   1 = write(int fd, const void *buf, len)  → bytes written
;   2 = read(int fd, void *buf, len)         → bytes read
;   3 = getpid()                             → PID
;   4 = brk(void *addr)                      → new program break
;
; ── Memory Layout ────────────────────────────────────────────────────────────
;   0x200_0000  code + rodata  (_start entry, PAGE_USER_RW)
;   0x200_1000  user stack (grows down, 4 KiB, RSP = 0x200_2000)
;   0x201_0000  heap start (brk)
;
; ── Echo Spec ────────────────────────────────────────────────────────────────
;   echo [OPTIONS] [args...]
;     -n   omit trailing newline
;     -e   enable escape interpretation (\n, \t, \r, \\, \0NNN)
;     -E   disable escapes (default)
;     --   end of options
;   Exit code 0
;==============================================================================

ORG 0x2000000
bits 64

; ── Constants ────────────────────────────────────────────────────────────────
SYS_EXIT        equ 0
SYS_WRITE       equ 1
STDOUT          equ 1
ESC_BUF_SIZE    equ 256             ; per-arg escape decode buffer

;==============================================================================
; _start — PID 1 entry point: runs echo test cases then exits
;==============================================================================
section .text

global _start
_start:
    mov rsp, 0x200_2000

    ; ── Test 1: echo hello world ──────────────────────────────────────────
    mov rdi, 3                          ; argc = 3
    lea rsi, [rel args1]                ; argv = ["echo", "hello", "world"]
    call echo

    ; ── Test 2: echo -n no-newline (suppress trailing \n) ─────────────────
    mov rdi, 3
    lea rsi, [rel args2]
    call echo

    ; ── Test 3: echo -e "tab\there" (interpret escapes) ───────────────────
    mov rdi, 3
    lea rsi, [rel args3]
    call echo

    ; ── Test 4: echo -E plain (explicitly disable escapes, default) ───────
    mov rdi, 3
    lea rsi, [rel args4]
    call echo

    ; ── Test 5: echo -- end of options (-- is consumed, not printed) ──────
    mov rdi, 5
    lea rsi, [rel args5]
    call echo

    ; ── Test 6: echo (empty args, just newline) ───────────────────────────
    mov rdi, 1
    lea rsi, [rel args6]
    call echo

    ; ── Test 7: echo -e "\0101there" (octal 0101 = 'A') ──────────────────
    mov rdi, 3
    lea rsi, [rel args7]
    call echo

    ; ── Exit cleanly ──────────────────────────────────────────────────────
    xor rdi, rdi                        ; code = 0
    mov rax, SYS_EXIT
    syscall

;==============================================================================
; Read-only data — test argument arrays and string constants
;==============================================================================
section .rodata

; ── Argument pointer arrays (each ends at argc count) ────────────────────────
args1:  dq str_echo, str_hello, str_world               ; echo hello world
args2:  dq str_echo, str_n_flag, str_no_newline         ; echo -n no-newline
args3:  dq str_echo, str_e_flag, str_tab_here           ; echo -e "tab\there"
args4:  dq str_echo, str_E_flag, str_plain              ; echo -E plain
args5:  dq str_echo, str_end_flag, str_of, str_end, str_options  ; echo -- end of options
args6:  dq str_echo                                     ; echo (just command)
args7:  dq str_echo, str_e_flag, str_octal_test         ; echo -e "\0101there"

; ── String literals (null-terminated, ASCII) ─────────────────────────────────
str_echo:        db "echo", 0
str_hello:       db "hello", 0
str_world:       db "world", 0
str_n_flag:      db "-n", 0
str_no_newline:  db "no-newline", 0
str_e_flag:      db "-e", 0
str_tab_here:    db "tab\there", 0          ; literal backslash-t (2 chars)
str_E_flag:      db "-E", 0
str_plain:       db "plain", 0
str_end_flag:    db "--", 0
str_of:          db "of", 0
str_end:         db "end", 0
str_options:     db "options", 0
str_octal_test:  db "\0101there", 0         ; literal "\0101" then "there"
; str_space and str_newline are defined by vibix_echo.inc

; ── Shared echo implementation ────────────────────────────────────────────────
%include "vibix_echo.inc"
