;==============================================================================
; cat — copy stdin to stdout
;==============================================================================
ORG 0x2000000
bits 64
section .text
global _start
_start:
    call cat
    xor rdi, rdi
    mov rax, 0
    syscall

section .rodata
%include "vibix_cat.inc"
