OUTPUT_FORMAT(elf32-i386)
ENTRY(start)

MEMORY {
    boot : org = 0x7c00,  l = 512 /* bootloader */
    ram  : org = 0x10000, l = 64K /* kernel */
}

SECTIONS {
    .boot : {
        *(.boot)
    } >boot

    .text : {
        *(.text*)
    } >ram

    .data : { *(.data*) } >ram
    .rodata : { *(.rodata*) } >ram
}
