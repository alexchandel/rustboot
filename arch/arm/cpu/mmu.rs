use core;

use kernel::mm::physical;
use kernel::mm::physical::Phys;

pub type Frame = [u8, ..PAGE_SIZE];

static PAGE_SIZE: uint = 0x1000;
static PAGE_SIZE_LOG2: uint = 12;

// kinda clever
define_flags!(Flags: u32 {
    SECTION = 0b10010,

    BUFFER = 1 << 2,
    CACHE,
    RW     = 1 << 10,
    CLIENT_ACCESS
})

#[packed]
pub struct Descriptor(u32);

#[packed]
struct PageTableCoarse {
    pages: [Descriptor, ..256]
}

#[allow(visible_private_types)]
#[packed]
pub struct PageDirectory {
    entries: [Descriptor, ..4096]
}

pub static mut directory: *mut PageDirectory = 0 as *mut PageDirectory;

define_reg!(CR, CRFlags: uint {
    CR_M  = 1 << 0,  // MMU enable
    CR_A  = 1 << 1,
    CR_C  = 1 << 2,  // Data cache enable
    CR_W  = 1 << 3,
    CR_P  = 1 << 4,  // 32-bit exception handler
    CR_D  = 1 << 5,  // 32-bit data address range
    CR_L  = 1 << 6,  // Implementation defined
    CR_B  = 1 << 7,  // Endianness
    CR_S  = 1 << 8,
    CR_R  = 1 << 9,
    CR_F  = 1 << 10, // Implementation defined
    CR_Z  = 1 << 11, // Implementation defined
    CR_I  = 1 << 12, // Instruction cache enable
    CR_V  = 1 << 13,
    CR_RR = 1 << 14,
    CR_L4 = 1 << 15
})

// Each of the 16 domains can be either allowed full access (manager)
// to a region of memory or restricted access to some pages in that region (client).
define_flags!(DomainTypeMask: uint {
    KERNEL = 0b11 << 0,
    USER   = 0b11 << 2,
    NOACCESS = 0,
    CLIENT   = 0b01 * 0x55555555,
    MANAGER  = 0b11 * 0x55555555
})

impl CR {
    #[inline] #[allow(dead_code)]
    pub fn read() -> CRFlags {
        unsafe {
            let flags;
            asm!(concat!("mrc p15, 0, $0, c1, c0, 0") : "=r"(flags));
            CRFlags(flags)
        }
    }

    #[inline] #[allow(dead_code)]
    pub fn write(f: CRFlags) {
        match f {
            CRFlags(val) => unsafe {
                asm!(concat!("mcr p15, 0, $0, c1, c0, 0") :: "r"(val) :: "volatile");
            }
        }
    }
}

pub unsafe fn init() {
    let dir: Phys<PageDirectory> = physical::zero_alloc_frames(4);

    (*dir.as_ptr()).entries[0] = Descriptor::section(0, RW);

    switch_directory(dir);
    enable_paging();
}

pub fn switch_directory(dir: Phys<PageDirectory>) {
    // Memory protection is determined by control register c1 bits S and R,
    // domain access reg. c3 and per-page domain number and permission bits.
    let cpu_domain = KERNEL & MANAGER | USER & MANAGER;

    unsafe {
        asm!("mcr p15, 0, $0, c3, c0, 0     // load domain access register
              mcr p15, 0, $1, c2, c0, 0     // load page table pointer
            " :: "r"(cpu_domain), "r"(dir.as_ptr()) : "ip" : "volatile"); // dir.offset()?
    }
}

fn enable_paging() {
    unsafe {
        asm!("mov ip, 0
              mcr p15, 0, ip, c7, c5, 0     // invalidate I & D cache
              mcr p15, 0, ip, c7, c10, 4    // drain write buffer
              mcr p15, 0, ip, c8, c7, 0     // invalidate I & D TLBs
            " ::: "ip" : "volatile");

        CR::write(CR - (CR_A | CR_W | CR_P | CR_D | CR_R | CR_F | CR_Z | CR_V | CR_RR)
                     | (CR_S | CR_I | CR_C | CR_M));
    }
}

pub unsafe fn map(_: *mut u8, _: uint, _: Flags) {
    // TODO
}

impl Descriptor {
    fn section(base: u32, flags: Flags) -> Descriptor {
        // make a section descriptor
        Descriptor(base) | flags | SECTION
    }
}

impl_ops!(Descriptor, Flags)

impl PageDirectory {
    pub unsafe fn map(&self, _: *mut u8, _: uint, _: Flags) {
        // TODO
    }

    pub unsafe fn clone(&mut self) -> Phys<PageDirectory> {
        Phys::at(self as *mut PageDirectory as uint)
    }
}

/// STUB
pub fn clone_directory() -> Phys<PageDirectory> {
    unsafe {
        !fail!("Unimplemented");
    }
}
