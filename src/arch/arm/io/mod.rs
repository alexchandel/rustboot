use core::intrinsics::volatile_store;
use core::fmt;
use core::prelude::*;

pub static UART0: *mut u32 = 0x101f1000u as *mut u32;
pub static UART0_IMSC: *mut u32 = (0x101f1000u + 0x038u) as *mut u32;

pub unsafe fn write_word(c: u32) {
    volatile_store(UART0, c);
}

pub unsafe fn write_char(c: char) {
    volatile_store(UART0, c as u32);
}

pub fn putc(c: u32) {
	unsafe {
		write_word(c);
	}
}


struct Stdout;

impl Stdout {
    fn write_fmt(&mut self, fmt: &fmt::Arguments) {
        fmt::write(self, fmt);
    }
}

impl fmt::FormatWriter for Stdout {
    fn write(&mut self, bytes: &[u8]) -> fmt::Result {
        for &c in bytes.iter() {
            putc(c as u32);
        }
        Ok(())
    }
}

pub fn print_args(fmt: &fmt::Arguments) {
    write!(Stdout, "{}", fmt);
}

pub fn println_args(fmt: &fmt::Arguments) {
    writeln!(Stdout, "{}", fmt);
}
