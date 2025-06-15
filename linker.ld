ENTRY(_start)

KERNEL_BASE = 0xFFFFFFFF80000000;

PHDRS {
    text PT_LOAD FLAGS(5);    /* RX */
    rodata PT_LOAD FLAGS(4);  /* R */
    data PT_LOAD FLAGS(6);    /* RW */
    bss PT_LOAD FLAGS(6);     /* RW */
}

SECTIONS {
    . = KERNEL_BASE;

    .text : ALIGN(4K) {
        *(.text .text.*)
    } :text

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    } :rodata

    .data : ALIGN(4K) {
        *(.data .data.*)
    } :data

    .bss : ALIGN(4K) {
        *(.bss .bss.*)
        *(COMMON)
    } :bss

    /DISCARD/ : {
        *(.eh_frame)
    }
}
