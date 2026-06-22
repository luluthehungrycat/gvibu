;==============================================================================
; true — exit with code 0
;==============================================================================
ORG 0x2000000
bits 64
section .text
global _start
_start:
    call true_cmd           ; does exit(0) internally, never returns

section .rodata
%include "vibix_tiny.inc"
