//! Programmable interrupt controller

use cpu::io;

pub fn remap() {
    io::out(0x20, 0x11u16); // WARNING verify should be u16
    io::out(0xA0, 0x11u16);

    io::out(0x21, 0x20u16);
    io::out(0xA1, 0x28u16);

    io::out(0x21, 4u16);
    io::out(0xA1, 2u16);

    io::out(0x21, 1u16);
    io::out(0xA1, 1u16);
}

pub fn enable(irq: u8) {
    let port: u16 = if (irq & 0b1000) == 0 { 0x21 } else { 0xa1 };
    let mask: u8 = !(1u8 << (irq & 0b111) as uint);

    io::out(port, io::inb(port) & mask);
}

pub fn mask(mask: u16) {
    io::out(0x21, (mask & 0xFF) as u8);
    io::out(0xA1, ((mask >> 8) & 0xFF) as u8);
}
