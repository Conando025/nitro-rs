#![no_std]
#![no_main]

mod text;

use core::{
    // arch::asm,
    panic::PanicInfo,
    ptr::{self, addr_of},
};
use text::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = ".text.irq")]
#[unsafe(no_mangle)]
pub extern "C" fn _irq_handler() {}

unsafe extern "C" {
    #[link_name = "_bss_start"]
    static BSS_START: u32;
    #[link_name = "_bss_end"]
    static BSS_END: u32;
}

static EXAMPLE: [u32; 10] = [0; 10];

#[unsafe(no_mangle)]
pub extern "C" fn _entry() {
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
    loop {}
}

#[inline(never)]
#[instruction_set(arm::t32)]
fn get_vcount() -> u16 {
    unsafe {
        return ptr::read_volatile(0x4000006 as *const u16);
    }
}
