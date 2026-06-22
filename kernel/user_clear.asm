;==============================================================================
; clear — clear the terminal using ANSI escape codes
;==============================================================================
ORG 0x2000000
bits 64
section .text
global _start
_start:
    call clear_cmd
    xor rdi, rdi
    mov rax, 0
    syscall

section .rodata
%include "vibix_clear.inc"
