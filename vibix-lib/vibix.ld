/* VIBIX flat binary linker script
 *
 * Programs load at 0x2000000 (physical flat address space).
 * Stack begins at 0x2002000 (set by _start), grows downward.
 *
 * Usage:
 *   cargo rustc --target x86_64-unknown-none --release -- \
 *     -C link-arg=-Tvibix.ld -C link-arg=-nostartfiles
 *   rust-objcopy -O binary target/.../vibix vibix.bin
 */

OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

SECTIONS
{
    . = 0x2000000;

    .text : ALIGN(16) {
        *(.text .text.*)
    }

    .rodata : ALIGN(16) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(16) {
        *(.data .data.*)
    }

    .bss : ALIGN(16) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* Strip unnecessary ELF sections — they waste space in the flat binary */
    /DISCARD/ : {
        *(.comment)
        *(.note.*)
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.debug*)
    }
}
