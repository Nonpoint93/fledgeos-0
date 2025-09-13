
pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

#[allow(unused)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    White = 0xF,
    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    Yellow = 0xE,
    DarkGray = 0x8,
}

pub fn clear_screen(color: u8) {
    let framebuffer = 0xb8000 as *mut u8;
    for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
        unsafe {
            framebuffer.add(i * 2).write_volatile(b' ');
            framebuffer.add(i * 2 + 1).write_volatile(color);
        }
    }
}