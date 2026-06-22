;==============================================================================
; user_echo.asm — Echo flat binary for VIBIX (command entry point)
;
; Invoked as: echo(rdi=argc, rsi=argv)
; Supports: -n, -e, -E, --, escape decoding (\n, \t, \r, \\, \0NNN)
;==============================================================================
ORG 0x2000000
bits 64

section .text
global _start
_start:
    mov rsp, 0x2002000

    ; Forward argc/argv directly to echo and exit with its return value
    call echo                           ; echo(rdi, rsi)

    ; exit(0)
    xor rdi, rdi
    mov rax, 0                          ; SYS_EXIT
    syscall

; ── Shared echo implementation ───────────────────────────────────────────────
section .rodata
%include "vibix_echo.inc"
