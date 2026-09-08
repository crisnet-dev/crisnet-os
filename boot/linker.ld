ENTRY(crisnet_os_main)

SECTIONS
{
    . = 0x1000;

    .text : ALIGN(1) {
        *(.text.crisnet_os_main)
        *(.text*)
    }

    .rodata : ALIGN(1) {
        *(.rodata*)
    }

    .data : ALIGN(1) {
        *(.data*)
    }

    .bss : ALIGN(1) {
        *(.bss*)
        *(COMMON)
    }
}
