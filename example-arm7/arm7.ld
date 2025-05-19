ENTRY(_entry)


MEMORY {
  RAM     : org = 0x02380000, len = 512K
}

SECTIONS {
  .text : { *(.text); *(.text.*) } > RAM
  .data : { *(.data); *(.data.*) } > RAM
  .rodata : { *(.rodata); *(.rodata.*) } > RAM
  .bss ALIGN(4) : {
    _bss_start = ABSOLUTE(.);
    *(.bss); 
    *(.bss.*);
    . = ALIGN(4);
    _bss_end = ABSOLUTE(.);
  } > RAM
}
