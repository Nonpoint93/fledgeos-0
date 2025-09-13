#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

use core::arch::asm;
use core::panic::PanicInfo;
use core::fmt::Write;

use crate::cursor::Cursor;
use crate::vga::clear_screen;

mod vga;
mod cursor;

/// Custom panic handler for `no_std` environments.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    
    let mut cursor = Cursor {
        position: 0,
        foreground: vga::Color::White,
        background: vga::Color::Red,
    };
    clear_screen(cursor.color());
    cursor.position = 0;
    let _ = write!(cursor, "{}", info);
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Entry point of the program.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {

    let text = b"Rust in Action";

    let mut cursor: Cursor = Cursor{
        position: 0,
        foreground: vga::Color::BrightCyan,
        background: vga::Color::Black,
    };

    cursor.print(text);

    loop {
            unsafe{
                asm!("hlt");
            }
    }
}
