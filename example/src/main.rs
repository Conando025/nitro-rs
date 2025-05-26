#![no_std]
#![no_main]

mod text;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    ptr::{self, addr_of},
};
use text::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    #[link_name = "__bss_start"]
    static BSS_START: u32;
    #[link_name = "__bss_end"]
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
    ldr sp, =__irq_stack",
    // Supervisor mode
    "mov r0, #0x13
    msr cpsr, r0
    ldr sp, =__swi_stack",
    // System mode
    "mov r0, #0x1f
    msr cpsr, r0
    ldr sp, =__sys_stack",
    // Set irq
    "ldr r0, =irq_handler
    ldr r1, =__irq_handler
    str r0, [r1]",
    "b _entry_arm9",
);

#[unsafe(no_mangle)]
extern "C" fn _entry_arm9() -> ! {
    unsafe {
        init_bss();
    }

    main9();

    loop {}
}

unsafe fn init_bss() {
    unsafe {
        let bss_start = addr_of!(BSS_START) as *mut u32;
        let bss_size = bss_start.offset_from_unsigned(addr_of!(BSS_END));
        primative_memset(bss_start, bss_size, 0);
    }
}

unsafe fn primative_memset(start: *mut u32, len: usize, value: u32) {
    unsafe {
        let mem_region = core::slice::from_raw_parts_mut(start, len);
        for word in mem_region {
            *word = value;
        }
    }
}

fn main9() {
    unsafe {
        ptr::write_volatile(0x04000304 as *mut u32, 0x0000_8003);
        ptr::write_volatile(0x04000000 as *mut u32, 0x0002_0000);
        ptr::write_volatile(0x04000240 as *mut u8, 0x80);
        ptr::write_volatile(0x04000208 as *mut u32, 1);
    }

    return;

    let mut text = TextDisplay::init();

    text.write(addr_of!(BSS_START) as u32);
    text.write(addr_of!(BSS_END) as u32);
    text.write(addr_of!(EXAMPLE) as u32);
    unsafe {
        asm!("swi 0x05");
    }
    text.write((irq_handler as *const u32) as u32);
    text.new_line();
    for i in 0..10 {
        text.write(EXAMPLE[i]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn irq_handler() {
    unsafe {
        ptr::write_volatile(addr_of!(EXAMPLE) as *mut u32, 1);
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
    ldr sp, =__irq_stack",
    // Supervisor mode
    "mov r0, #0x13
    msr cpsr, r0
    ldr sp, =__swi_stack",
    // System modeBerechnen Sie die Matrix-Vektor Produkte!
    "mov r0, #0x1f
    msr cpsr, r0
    ldr sp, =__sys_stack",
    "b _entry_arm7",
);

#[unsafe(no_mangle)]
extern "C" fn _entry_arm7() -> ! {
    unsafe {
        init_bss();
    }

    main7();

    loop {}
}

#[unsafe(no_mangle)]
fn main7() {}
