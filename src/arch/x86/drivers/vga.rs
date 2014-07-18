use core::mem::transmute;

use cpu::io;
use platform::runtime::wmemset;

#[repr(u8)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

#[packed]
struct Char {
    pub char: u8,
    attr: u8,
}

impl Char {
    #[inline]
    pub fn new(c: char, fg: Color, bg: Color) -> Char {
        Char { char: c as u8, attr: fg as u8 | (bg as u8 << 4) }
    }
}

pub static SCREEN_SIZE: uint = 80*25;
type Screen = [Char, ..SCREEN_SIZE];
pub static SCREEN: *mut Screen = 0xb8000 as *mut Screen;

pub fn clear_screen(bg: Color) {
    unsafe {
        wmemset(SCREEN as *mut u8, transmute(Char::new(' ', Black, bg)), SCREEN_SIZE);
    }
}

pub fn cursor_at(pos: uint) {
    io::out(0x3D4, 15u16); // WARNING verify should be u16
    io::out(0x3D5, pos as u8);
    io::out(0x3D4, 14u16);
    io::out(0x3D5, (pos >> 8) as u8);
}
