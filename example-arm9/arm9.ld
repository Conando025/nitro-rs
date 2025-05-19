ENTRY(_entry)

MEMORY {
  RAM   : org = 0x02000000, len = 4M - 512k
}

SECTIONS {
  .text ALIGN(4): { *(.text); *(.text.*) } > RAM
  .data ALIGN(4) : { *(.data); *(.data.*) } > RAM
  .rodata ALIGN(4) : { *(.rodata); *(.rodata.*) } > RAM
  .bss ALIGN(4) : {
    _bss_start = ABSOLUTE(.);
    *(.bss); 
    *(.bss.*);
    . = ALIGN(4);
    _bss_end = ABSOLUTE(.);
  } > RAM
}
