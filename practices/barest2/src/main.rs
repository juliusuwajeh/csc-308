#![no_std]
#![no_main]

use x86_64::instructions::hlt;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

pub struct FrameBufferWriter {
    framebuffer: *mut u8,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
    color: u8,
}

impl FrameBufferWriter {
    /// Initializes the FrameBufferWriter
    pub fn new(framebuffer: *mut u8, width: usize, height: usize) -> Self {
        Self {
            framebuffer,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            color: 0x0f, // White text on black background
        }
    }

    /// Sets the cursor position dynamically
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cursor_x = x;
            self.cursor_y = y;
        } else {
            // Log or handle invalid cursor positions gracefully
            self.cursor_x = 0;
            self.cursor_y = self.height - 1; // Move to the last line
        }
    }

    /// Writes a single character to the screen
    pub fn write_char(&mut self, c: u8) {
        // If the cursor position exceeds the bounds, wrap or scroll
        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
        if self.cursor_y >= self.height {
            self.scroll_up();
            self.cursor_y = self.height - 1;
        }

        let position = self.cursor_y * self.width + self.cursor_x;
        unsafe {
            *self.framebuffer.offset(position as isize * 2) = c; // ASCII character
            *self.framebuffer.offset(position as isize * 2 + 1) = self.color; // Attribute byte
        }

        self.cursor_x += 1; // Move the cursor to the right
    }

    /// Scrolls the screen up by one line
    pub fn scroll_up(&mut self) {
        let row_size = self.width * 2;
        unsafe {
            for y in 1..self.height {
                let src = self.framebuffer.offset((y * self.width * 2) as isize);
                let dst = self.framebuffer.offset(((y - 1) * self.width * 2) as isize);
                for x in 0..row_size {
                    *dst.offset(x as isize) = *src.offset(x as isize);
                }
            }

            // Clear the last row
            let last_row = self.framebuffer.offset(((self.height - 1) * self.width * 2) as isize);
            for x in 0..row_size {
                *last_row.offset(x as isize) = 0;
            }
        }
    }

    /// Writes a string to the screen
    pub fn write_string(&mut self, text: &str) {
        for &byte in text.as_bytes() {
            match byte {
                b'\n' => {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
                _ => self.write_char(byte),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    let mut writer = FrameBufferWriter::new(framebuffer, 80, 25);

    // Testing dynamic cursor positioning
    writer.set_cursor(10, 5); // Move to (10, 5)
    writer.write_string("Hello, World!");

    // Testing overflow handling and screen scrolling
    writer.set_cursor(0, 24); // Move to the bottom row
    writer.write_string("This will scroll the screen!");

    loop {
        hlt();
    }
}
