ENTRY(__entry_arm9)

MEMORY {
  itcm       (rwx) : org = 0x01ff8000, len = 32k
  main_ram   (rwx) : org = 0x02000000, len = 4M - 512k
  dtcm       (rw)  : org = 0x027e0000, len = 16k
}

__dtcm_start = ORIGIN(dtcm);
__dtcm_end = ORIGIN(dtcm) + LENGTH(dtcm);
__irq_handler = __dtcm_end - 0x4;
__irq_check_bits = __irq_handler - 0x4;
__irq_stack = __irq_check_bits - 0x100;
__swi_stack = __irq_stack - 0x100;
__sys_stack = __swi_stack - 0x100;

SECTIONS {
  .text ALIGN(4): { *(.text); *(.text.*) } > main_ram
  .data ALIGN(4) : { *(.data); *(.data.*) } > main_ram
  .rodata ALIGN(4) : { *(.rodata); *(.rodata.*) } > main_ram
  .bss ALIGN(4) : {
    __bss_start = ABSOLUTE(.);
    *(.bss); 
    *(.bss.*);
    . = ALIGN(4);
    __bss_end = ABSOLUTE(.);
  } > main_ram
  __heap_start = ABSOLUTE(.);
}
