ENTRY(reset)

MEMORY {
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS {
    .vector_table : { KEEP(*(.vector_table)) } > FLASH
    .text : { *(.text*) *(.rodata*) } > FLASH
    .data : {
        _sdata = .;
        *(.data*)
        _edata = .;
    } > RAM AT > FLASH
    _sidata = LOADADDR(.data);
    .bss (NOLOAD) : {
        _sbss = .;
        *(.bss*) *(COMMON)
        _ebss = .;
    } > RAM
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
