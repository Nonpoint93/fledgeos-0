#![no_std]
#![no_main]
#![feature(core_intrinsics)]

//! # Minimal Bare-Metal Rust Program
//! This program runs without the standard library and without a main function.
//! It writes a character to the VGA text buffer and enters an infinite loop.
//! Useful as a starting point for OS development or embedded systems.

use core::intrinsics;
use core::panic::PanicInfo;

/// Custom panic handler for `no_std` environments.
/// This function is called when a panic occurs.
/// It aborts the program using a core intrinsic.
///
/// # Arguments
/// * `_info` - Information about the panic (unused here).
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        intrinsics::abort();
    }
}

/// Entry point of the program.
/// This function writes a character to the VGA framebuffer
/// and then enters an infinite loop.
///
/// # Safety
/// Direct memory access is used to write to VGA memory.
/// This assumes the program is running in an environment
/// where 0xb8000 points to the VGA text buffer.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    unsafe {
        framebuffer
            .offset(1)
            .write_volatile(0x30); // ASCII '0' with default color
    }

    loop {}
}
