;==============================================================================
; printenv — print all or selected environment variables
;==============================================================================
ORG 0x2000000
bits 64

section .text
global _start
_start:
    ; envp = argv[argc + 1] (follows argv in the stack layout)
    lea rdx, [rsi + rdi*8 + 8]  ; rdx = envp
    call printenv                ; printenv(rdi=argc, rsi=argv, rdx=envp)

    xor rdi, rdi
    mov rax, 0
    syscall

section .rodata
%include "vibix_core.inc"
%include "vibix_printenv.inc"
