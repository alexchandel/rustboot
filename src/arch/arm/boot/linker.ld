OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(start)

SECTIONS {
    . = 0x10000;

    .text : {
       build/arm/loader.o
       *(.text .rodata)
    }

    .data : { *(.data) }
    .bss : { *(.bss) }
}
