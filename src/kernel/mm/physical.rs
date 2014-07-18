use core::mem::transmute;
use core::ptr::RawPtr;
use core::option::Option;

use kernel::heap;
use kernel::mm;
use kernel::mm::Allocator;
use cpu::mmu::Frame;
use util::bitv;

use rust_core::fail::abort;

pub static mut frames: mm::Alloc = mm::Alloc {
    base: 0x200_000 as *mut u8,
    el_size: 12,
    parent: mm::BuddyAlloc {
        order: 13,
        tree: bitv::Bitv { storage: 0 as *mut u32 }
    }
};

pub struct Phys<T> {
    ptr: *mut T
}

impl<T> Phys<T> {
    pub fn at(offset: uint) -> Phys<T> {
        Phys { ptr: offset as *mut T }
    }

    pub fn as_ptr(&self) -> *mut T {
        match *self {
            Phys { ptr: p } => p
        }
    }
}

impl<T> RawPtr<T> for Phys<T> {
    fn null() -> Phys<T> {
        Phys { ptr: RawPtr::null() }
    }

    fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    fn to_uint(&self) -> uint {
        self.ptr.to_uint()
    }

    unsafe fn to_option(&self) -> Option<&T> {
        self.ptr.to_option()
    }

    unsafe fn offset(self, n: int) -> Phys<T> {
        Phys { ptr: self.ptr.offset(n) }
    }
}

pub fn init() {
    unsafe {
        frames.parent.tree.storage = heap::zero_alloc::<u32>(1024);
    }
}

pub unsafe fn alloc_frames<T = Frame>(count: uint) -> Phys<T> {
    match frames.alloc(count) {
        (_, 0) => abort(),
        (ptr, _) => Phys { ptr: ptr as *mut T }
    }
}

pub unsafe fn zero_alloc_frames<T = Frame>(count: uint) -> Phys<T> {
    match frames.zero_alloc(count) {
        (_, 0) => abort(),
        (ptr, _) => Phys { ptr: ptr as *mut T }
    }
}

#[inline]
pub unsafe fn free_frames<T>(ptr: Phys<T>) {
    frames.free(ptr.to_uint() as *mut u8);
}
