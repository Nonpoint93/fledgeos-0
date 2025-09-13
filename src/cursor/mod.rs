use core::fmt;

use crate::vga::Color;

pub struct Cursor {
    pub position: usize,
    pub foreground: Color,
    pub background: Color,
}

impl Cursor {

    pub fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    pub fn print(&mut self, text: &[u8]) {
        
        let color: u8 = self.color();
        let framebuffer: *mut u8 = 0xb8000 as *mut u8;

        for &character in text {
            let offset: usize = self.position * 2;
            if self.position < crate::vga::BUFFER_WIDTH * crate::vga::BUFFER_HEIGHT {
                unsafe {
                    framebuffer.add(offset).write_volatile(character);
                    framebuffer.add(offset + 1).write_volatile(color);
                }
                self.position += 1;
            }
        }
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}