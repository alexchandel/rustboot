//! This module defines the 8-byte entries in the Interrupt Descriptor Table.

use core;

use super::DtReg;

define_flags!(IdtFlags: u8 {
    INTR_GATE = 0b1110,
    TRAP_GATE = 0b1111,
    PRESENT = 1 << 7
})

pub type IdtReg = DtReg<IdtEntry>;

#[packed]
pub struct IdtEntry {
    addr_lo: u16,
    sel: u16,       // a code segment selector in GDT or LDT
    zero: u8,       // unused, always 0!
    flags: IdtFlags,
    addr_hi: u16
}

impl IdtEntry {
    pub fn new(func: unsafe extern "C" fn(), sel: u16, flags: IdtFlags) -> IdtEntry {
        let addr = func as uint;
        let (addr_hi, addr_lo) = (
            (addr & 0xFFFF0000) >> 16,
            (addr & 0x____FFFF)
        );
        IdtEntry {
            addr_lo: addr_lo as u16,
            addr_hi: addr_hi as u16,
            sel: sel,
            zero: 0,
            flags: flags
        }
    }
}

impl super::DtReg<IdtEntry> {
    #[inline]
    pub fn load(&self) {
        unsafe {
            asm!("lidt [$0]" :: "A"(self) :: "intel");
        }
    }
}
