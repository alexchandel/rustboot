//! The Interrupt Table and Isr (Interrupt Service Routine) classes.

use core::mem::transmute;
use core::ptr::RawPtr;

use cpu::DtReg;
use cpu::exception::Fault;
use cpu::idt::{IdtEntry, IdtReg, INTR_GATE, PRESENT};
use platform::drivers::pic;
use kernel::heap;


pub enum Int {
    Fault(Fault)
}

pub struct Table {
    reg: &'static IdtReg,
    table: *mut IdtEntry,
    mask: u16,
}

impl Table {
    pub fn new() -> Table {
        unsafe {
            let table = heap::zero_alloc::<IdtEntry>(256);
            let reg = heap::alloc::<IdtReg>(1);
            *(reg as *mut IdtReg) = DtReg::new(table, 256);
            Table {
                reg: transmute(reg),
                table: table,
                mask: 0xffff
            }
        }
    }

    pub unsafe fn enable_maskable(&mut self, irq: uint, isr: unsafe extern "C" fn()) {
        *self.table.offset(irq as int) = IdtEntry::new(
            isr,                // interrupt service routine
            1 << 3,             // segment selector
            INTR_GATE | PRESENT // flags
        );

        self.mask &= !(1u16 << (irq & 0b1111));
        pic::mask(self.mask);
    }

    #[allow(visible_private_types)]
    pub unsafe fn set_isr(&mut self, val: Fault, code: bool, handler: unsafe extern "C" fn()) {
        *self.table.offset(val as int) = Isr::new(Fault(val), code).idt_entry(handler);
    }

    pub fn load(&self) {
        self.reg.load();
        pic::remap();
        pic::mask(self.mask);
        enable();
    }
}

fn enable() {
    unsafe {
        asm!("sti" :::: "volatile", "intel");
    }
}

#[packed]
pub struct Isr {
    push_dummy: u8, // push eax  // (only for exceptions without error codes)
    push: u8,       // push byte <imm>  // save int. number
    value: Int,
    jmp: u8,        // jmp rel  // jump to the common handler
    rel: i32
}

impl Isr {
    pub fn new(val: Int, code: bool) -> &mut Isr {
        let this: &mut Isr = unsafe { transmute(heap::alloc::<Isr>(1)) };
        *this = Isr {
            push_dummy: if code { 0x90 } else { 0x50 },   // [9]
            push: 0x6a, value: val,
            jmp: 0xe9, rel: -5
        };
        this
    }

    pub unsafe fn idt_entry(&mut self, handler: unsafe extern "C" fn()) -> IdtEntry {
        self.rel = handler as i32 - (self as *mut Isr).offset(1) as i32;
        IdtEntry::new(transmute(self), 1 << 3, INTR_GATE | PRESENT)
    }
}
