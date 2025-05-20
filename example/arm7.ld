ENTRY(_entry_arm7)


MEMORY {
  RAM    (rwx) : org = 0x02380000, len = 512K
  WRAM   (rw)  : org = 0x03800000, len = 64K
}

__wram_end = (ORIGIN(WRAM) + LENGTH(WRAM));
__irq_handler = __wram_end - 0x4;
__irq_check_bits = __irq_handler - 0x4;
__irq_stack = __irq_check_bits - 0x100;
__swi_stack = __irq_stack - 0x100;
__sys_stack = __swi_stack - 0x100;

SECTIONS {
  .text : { *(.text); *(.text.*) } > RAM
  .data : { *(.data); *(.data.*) } > RAM
  .rodata : { *(.rodata); *(.rodata.*) } > RAM
  .bss ALIGN(4) : {
    __bss_start = ABSOLUTE(.);
    *(.bss); 
    *(.bss.*);
    . = ALIGN(4);
    __bss_end = ABSOLUTE(.);
  } > RAM
}
