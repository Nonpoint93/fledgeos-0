#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

use core::arch::asm;
use core::panic::PanicInfo;

//mod vga;

/// Custom panic handler for `no_std` environments.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Entry point of the program.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    unsafe {
        framebuffer.offset(1).write_volatile(0x30);
    }
    loop {
        unsafe{
            asm!("hlt");
        }
    }
}
