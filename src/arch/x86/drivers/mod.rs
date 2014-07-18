use core::option::{Option, None};

use kernel;

pub mod pic;
pub mod vga;
pub mod keyboard;

pub static mut keydown: Option<fn(u8)> = None;

pub fn init() {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);

    unsafe {
        kernel::int_table.map(|mut t| {
            t.enable_maskable(keyboard::IRQ, keyboard::isr_addr());
        });
    }
}
