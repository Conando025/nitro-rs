ENTRY(__entry_arm9)

MEMORY {
  itcm       (rwx) : org = 0x01ff8000, len = 32k
  main_ram   (rwx) : org = 0x02000000, len = 4M - 512k
  dtcm       (rw)  : org = 0x027e0000, len = 16k
}

_dtcm_start = ORIGIN(dtcm);
_dtcm_end = ORIGIN(dtcm) + LENGTH(dtcm);
_irq_stack = _dtcm_end - 0x100;
_swi_stack = _irq_stack - 0x100;
_sys_stack = _swi_stack - 0x100;

SECTIONS {
  .text ALIGN(4): { *(.text); *(.text.*) } > main_ram
  .data ALIGN(4) : { *(.data); *(.data.*) } > main_ram
  .rodata ALIGN(4) : { *(.rodata); *(.rodata.*) } > main_ram
  .bss ALIGN(4) : {
    _bss_start = ABSOLUTE(.);
    *(.bss); 
    *(.bss.*);
    . = ALIGN(4);
    _bss_end = ABSOLUTE(.);
  } > main_ram
  _heap_start = ABSOLUTE(.);
}
