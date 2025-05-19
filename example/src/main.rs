#![no_std]
#![no_main]

mod text;

use core::{
    arch::global_asm,
    panic::PanicInfo,
    ptr::{self, addr_of},
};
use text::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    #[link_name = "_bss_start"]
    static BSS_START: u32;
    #[link_name = "_bss_end"]
    static BSS_END: u32;
}

static EXAMPLE: [u32; 10] = [0; 10];

global_asm!(
    ".global __entry_arm9",
    "__entry_arm9:",
    "mov r0, #0x04000000",
    "str r0, [r0, #208]",
    // IRQ mode
    "mov r0, #0x12
    msr cpsr, r0
    ldr sp, =_irq_stack",
    // Supervisor mode
    "mov r0, #0x13
    msr cpsr, r0
    ldr sp, =_swi_stack",
    // System mode
    "mov r0, #0x1f
    msr cpsr, r0
    ldr sp, =_sys_stack",
    "b _entry_arm9",
);

#[unsafe(no_mangle)]
extern "C" fn _entry_arm9() -> ! {
    init_bss();

    main9();

    loop {}
}

fn init_bss() {
    unsafe {
        let bss_size: usize = addr_of!(BSS_END).offset_from_unsigned(addr_of!(BSS_START));
        let bss = core::slice::from_raw_parts_mut(BSS_START as *mut u32, bss_size);
        for word in bss {
            *word = 0;
        }
    }
}

fn main9() {
    unsafe {
        ptr::write_volatile(0x04000304 as *mut u32, 0x0000_8003);
        ptr::write_volatile(0x04000000 as *mut u32, 0x0002_0000);
        ptr::write_volatile(0x04000240 as *mut u8, 0x80);
        ptr::write_volatile(0x04000208 as *mut u32, 1);
        ptr::write_volatile(0x04000210 as *mut u32, 1);
    }

    let mut text = TextDisplay::init();

    text.write(addr_of!(BSS_START) as u32);
    text.write(addr_of!(BSS_END) as u32);
    text.write(addr_of!(EXAMPLE) as u32);
    text.new_line();
    for i in 0..10 {
        text.write(EXAMPLE[i]);
    }
}

global_asm!(
    ".global __entry_arm7",
    "__entry_arm7:",
    "mov r0, #0x04000000",
    "str r0, [r0, #208]",
    // IRQ mode
    "mov r0, #0x12
    msr cpsr, r0
    ldr sp, =_irq_stack",
    // Supervisor mode
    "mov r0, #0x13
    msr cpsr, r0
    ldr sp, =_swi_stack",
    // System mode
    "mov r0, #0x1f
    msr cpsr, r0
    ldr sp, =_sys_stack",
    "b _entry_arm7",
);

#[unsafe(no_mangle)]
extern "C" fn _entry_arm7() -> ! {
    init_bss();

    main7();

    loop {}
}

#[unsafe(no_mangle)]
fn main7() {}
