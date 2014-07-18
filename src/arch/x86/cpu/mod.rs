use core::mem::size_of;
use core::option::{Option, None, Some};
use core;

use kernel::heap;
use kernel;

mod gdt;
mod idt;
mod tss;
pub mod interrupt;
pub mod io;
mod exception;
pub mod mmu;

macro_rules! cpuid(
    ($n:expr, $s1:expr, $s2:expr, $s3:expr, $s4:expr) => (
        asm!("cpuid"
            : "=A"($s1),
              "={ebx}"($s2),
              "={edx}"($s3),
              "={ecx}"($s4)
            : "A"($n) :: "intel");
    );
    ($n:expr, *$s1:expr) => (
        cpuid!($n, (*$s1)[0], (*$s1)[1], (*$s1)[2], (*$s1)[3]);
    );
    ($n:expr, $s1:expr) => (
        asm!("cpuid"
            : "=A"($s1)
            : "A"($n) : "ebx", "edx", "ecx" : "intel");
    );
)

define_flags!(Eflags: u32 {
    CF,
    IF = 1 << 9
})

impl Eflags {
    fn read() -> Eflags {
        unsafe {
            let mut flags: u32;
            asm!("pushf; pop $0;" : "=r"(flags) ::: "volatile")
            Eflags(flags)
        }
    }
}

define_flags!(CR0Flags: u32 {
    CR0_PG = 1 << 31
})

struct CR0;

// http://www.jaist.ac.jp/iscenter-new/mpc/altix/altixdata/opt/intel/vtune/doc/users_guide/mergedProjects/analyzer_ec/mergedProjects/reference_olh/mergedProjects/instructions/instruct32_hh/vc178.htm
impl CR0 {
    #[inline]
    fn read() -> CR0Flags {
        unsafe {
            let flags;
            asm!("mov $0, cr0" : "=r"(flags) ::: "intel");
            CR0Flags(flags)
        }
    }

    #[inline]
    fn write(f: CR0Flags) {
        match f {
            CR0Flags(val) => unsafe {
                asm!("mov cr0, $0" :: "r"(val) :: "volatile", "intel");
            }
        }
    }
}

impl core::ops::BitOr<CR0Flags, CR0Flags> for CR0 {
    #[inline(always)]
    fn bitor(&self, other: &CR0Flags) -> CR0Flags {
        match (CR0::read(), other) {
            (CR0Flags(flags1), &CR0Flags(flags2)) => CR0Flags(flags1 | flags2)
        }
    }
}

struct CR3;

// http://www.jaist.ac.jp/iscenter-new/mpc/altix/altixdata/opt/intel/vtune/doc/users_guide/mergedProjects/analyzer_ec/mergedProjects/reference_olh/mergedProjects/instructions/instruct32_hh/vc178.htm
impl CR3 {
    #[inline]
    fn read() -> *mut mmu::PageDirectory {
        unsafe {
            let ptr;
            asm!("mov $0, cr3" : "=r"(ptr) ::: "intel");
            ptr
        }
    }

    #[inline]
    fn write(ptr: *mut mmu::PageDirectory) {
        unsafe {
            asm!("mov cr3, $0" :: "r"(ptr) :: "volatile", "intel");
        }
    }
}

#[packed]
struct DtReg<T> {
    size: u16,
    addr: *mut T,
}

impl<T> DtReg<T> {
    pub fn new(descriptor_table: *mut T, capacity: uint) -> DtReg<T> {
        DtReg {
            size: (capacity * size_of::<T>()) as u16,
            addr: descriptor_table,
        }
    }
}

// TODO: make push_dummy push ds?
// exception info and processor state saved on stack
struct Context {
    // Registers saved by the ISR (in reverse order)
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    ds: u32, es: u32, fs: u32, gs: u32,
    int_no: u32,   // added by ISRs
    err_code: u32, // added by some exceptions
    call_stack: IsrCallStack
}

// the cpu adds these when calling the ISR
struct IsrCallStack {
    eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32
}

impl Context {
    #[inline(always)]
    unsafe fn save() -> &mut Context {
        let this: &mut Context;
        asm!("push gs
              push fs
              .byte 0x06 // push es
              .byte 0x1e // push ds
              pusha"
            : "={esp}"(this) ::: "volatile", "intel");
        this
    }

    #[inline(always)]
    unsafe fn restore() {
        asm!("popa
              .byte 0x1f // pop ds
              .byte 0x07 // pop es
              pop fs
              pop gs
              add esp, 8
              iretd"
            :::: "volatile", "intel");
    }
}

struct LocalSegment {
    ts: tss::TssEntry,
}

impl LocalSegment {
    // FIXME: Rust needs address spaces
    fn get() -> &mut LocalSegment {
        unsafe {
            let this: &mut LocalSegment;
            asm!("mov $0, dword[gs:0]" : "=r"(this) ::: "volatile", "intel")
            this
        }
    }
}

pub static mut desc_table: Option<gdt::Gdt> = None;

/// Initializes the GDT, creates the ISR for breakpoints, calls the mmu::init()
/// which creates the ISR for Page Faults, and finally creates the ISR for
/// system calls.
pub fn init() {
    use cpu::gdt::{Gdt, GdtEntry, SIZE_32, STORAGE, CODE_READ, DATA_WRITE, DPL3};

    let local_data = unsafe {
        heap::zero_alloc::<LocalSegment>(1)
    };
    let tls = unsafe {
        let seg = heap::zero_alloc::<u32>(32);
        *seg = local_data as u32;
        // *(mut_offset(seg, 12)) = 0; // TODO: record stack bottom later
        seg
    };

    let t = Gdt::new();
    t.enable(1, GdtEntry::flat(STORAGE | CODE_READ, SIZE_32));
    t.enable(2, GdtEntry::flat(STORAGE | DATA_WRITE, SIZE_32));
    t.enable(3, GdtEntry::flat(STORAGE | CODE_READ | DPL3, SIZE_32));
    t.enable(4, GdtEntry::flat(STORAGE | DATA_WRITE | DPL3, SIZE_32));
    t.enable(5, GdtEntry::new(tls as u32, 32 * 4, STORAGE | DPL3, SIZE_32));
    unsafe {
        t.enable(6, (*local_data).ts.gdt_entry());
    }
    t.load(1 << 3, 2 << 3, 5 << 3);

    unsafe {
        desc_table = Some(t);

        kernel::int_table.map(|mut t| {
            use cpu::exception::{Breakpoint, exception_handler};
            t.set_isr(Breakpoint, false, exception_handler());
        });

        mmu::init();

        // Add the ISR for system calls.
        kernel::int_table.map(|mut t| {
            use cpu::exception::{SystemCall, exception_handler};
            t.set_isr(SystemCall, false, exception_handler());
        });
    }
}

pub fn info() {
}
