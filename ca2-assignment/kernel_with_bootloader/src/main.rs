#![no_std]
#![no_main]

extern crate alloc;

use bootloader_api::{config::Mapping, entry_point, BootInfo, BootloaderConfig};
use x86_64::instructions::hlt;
use log::warn;
use core::fmt::Write; // Import core::fmt module
use alloc::string::String;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

fn my_entry_point(boot_info: &'static mut BootInfo) -> ! {
    // Initialize a simple valid memory region for the allocator
    unsafe {
        ALLOCATOR.lock().init(0x100000, 0x10000); // 1MB region
    }

    // Framebuffer initialization
    let framebuffer_info = boot_info.framebuffer.as_mut().expect("No framebuffer found");
    let buffer_ptr = framebuffer_info.buffer().as_ptr() as *mut u8;
    let width = framebuffer_info.info().width;
    let height = framebuffer_info.info().height;

    let mut writer = FrameBufferWriter::new(buffer_ptr, width, height, 0x0F);

    // Use the enhanced print! macro
    print!(writer, "Hello, Rust!\nThis is a test.\tTabbed text.\nChanging color: \\cRed text.");

    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

pub struct FrameBufferWriter {
    buffer: *mut u8,
    width: usize,
    height: usize,
    position: (usize, usize),
    color_code: u8,
}

impl FrameBufferWriter {
    pub fn new(buffer: *mut u8, width: usize, height: usize, color_code: u8) -> Self {
        Self {
            buffer,
            width,
            height,
            position: (0, 0),
            color_code,
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\t' => self.write_tab(),
            b'\\' => self.handle_escape_sequence(byte),
            byte => {
                if self.position.0 >= self.width {
                    self.new_line();
                }

                let (x, y) = self.position;

                if x < self.width && y < self.height {
                    let index = y * self.width + x;
                    unsafe {
                        let buffer_ptr = self.buffer.add(index * 2);
                        *buffer_ptr = byte;
                        *buffer_ptr.add(1) = self.color_code;
                    }
                    self.position.0 += 1;
                } else {
                    // Handle invalid cursor positions gracefully
                    warn!("Attempt to write to invalid cursor position: ({}, {})", x, y);
                }
            }
        }
    }

    fn new_line(&mut self) {
        self.position.0 = 0;
        if self.position.1 + 1 < self.height {
            self.position.1 += 1;
        } else {
            self.scroll_up();
        }
    }

    fn write_tab(&mut self) {
        let spaces = 4 - (self.position.0 % 4); // Assuming a tab is 4 spaces
        for _ in 0..spaces {
            self.write_byte(b' ');
        }
    }

    fn handle_escape_sequence(&mut self, byte: u8) {
        // Handle custom escape sequences like \c for color change
        // This is just a simple example, you can extend it as needed
        if byte == b'c' {
            self.color_code = 0x0C; // Example: Change to red color
        } else {
            self.write_byte(byte);
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                let from_index = row * self.width + col;
                let to_index = (row - 1) * self.width + col;
                unsafe {
                    let from_ptr = self.buffer.add(from_index * 2);
                    let to_ptr = self.buffer.add(to_index * 2);
                    *to_ptr = *from_ptr;
                    *to_ptr.add(1) = *from_ptr.add(1);
                }
            }
        }

        // Clear the last row
        for col in 0..self.width {
            let index = (self.height - 1) * self.width + col;
            unsafe {
                let buffer_ptr = self.buffer.add(index * 2);
                *buffer_ptr = b' ';
                *buffer_ptr.add(1) = self.color_code;
            }
        }
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.position = (x, y);
        } else {
            warn!("Attempt to set invalid cursor position: ({}, {})", x, y);
        }
    }
}

// Define the print macro
#[macro_export]
macro_rules! print {
    ($writer:expr, $($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut temp_str = String::new();
            write!(&mut temp_str, $($arg)*).unwrap();
            $writer.write_string(&temp_str);
        }
    };
}
