;==============================================================================
; yes — repeatedly output "y\n" until killed
;==============================================================================
ORG 0x2000000
bits 64
section .text
global _start
_start:
    call yes_cmd            ; infinite loop, never returns

section .rodata
%include "vibix_tiny.inc"
