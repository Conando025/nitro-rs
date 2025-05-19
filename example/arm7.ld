ENTRY(_entry_arm7)


MEMORY {
  RAM    (rwx) : org = 0x02380000, len = 512K
  WRAM   (rw)  : org = 0x03800000, len = 64K
}

_irq_stack = (ORIGIN(WRAM) + LENGTH(WRAM)) - 0x100;
_swi_stack = _irq_stack - 0x100;
_sys_stack = _swi_stack - 0x100;

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
