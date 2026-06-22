;==============================================================================
; false — exit with code 1
;==============================================================================
ORG 0x2000000
bits 64
section .text
global _start
_start:
    call false_cmd          ; does exit(1) internally, never returns

section .rodata
%include "vibix_tiny.inc"
