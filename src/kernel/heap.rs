use core::mem::size_of;
use core::prelude::*;

use kernel::mm::{Allocator, Alloc, BuddyAlloc};
use util::bitv;

use rust_core::fail::{abort, out_of_memory};

pub static mut heap: Option<Alloc> = None;

pub fn init() -> Alloc {
    let alloc = Alloc::new(
        BuddyAlloc::new(17, bitv::Bitv { storage: 0x100_000 as *mut u32 }),
        0x110_000 as *mut u8,
        0,
    );
    unsafe {
        heap = Some(alloc);
    }
    alloc
}

#[lang = "exchange_malloc"]
#[inline]
pub unsafe fn malloc_raw(size: uint) -> *mut u8 {
    match get(heap).alloc(size) {
        (_, 0) => out_of_memory(),
        (ptr, _) => ptr
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_allocate(size: uint, _align: uint) -> *mut u8 {
    malloc_raw(size)
}

#[lang = "exchange_free"]
#[inline]
pub unsafe fn free<T>(ptr: *mut T) {
    get(heap).free(ptr as *mut u8);
}

#[inline]
pub unsafe fn alloc<T = u8>(count: uint) -> *mut T {
    match count.checked_mul(&size_of::<T>()) {
        None => out_of_memory(),
        Some(size) => malloc_raw(size) as *mut T
    }
}

#[inline]
pub unsafe fn zero_alloc<T = u8>(count: uint) -> *mut T {
    match count.checked_mul(&size_of::<T>()) {
        None => out_of_memory(),
        Some(size) => match get(heap).zero_alloc(size) {
            (_, 0) => out_of_memory(),
            (ptr, _) => ptr as *mut T
        }
    }
}

#[inline]
pub unsafe fn realloc_raw<T>(ptr: *mut T, count: uint) -> *mut T {
    match count.checked_mul(&size_of::<T>()) {
        None => out_of_memory(),
        Some(0) => {
            free(ptr as *mut u8);
            0 as *mut T
        }
        Some(size) => match get(heap).realloc(ptr as *mut u8, size) {
            (_, 0) => out_of_memory(),
            (ptr, _) => ptr as *mut T
        }
    }
}

// because no .expect() from lib std
fn get<T>(opt : Option<T>) -> T {
    match opt {
        Some(val) => val,
        None => abort(),
    }
}
