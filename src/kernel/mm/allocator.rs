use core::ptr::RawPtr;
use core::mem::transmute;
use core::ptr::{set_memory, copy_memory};
use core::intrinsics::offset;
use core::intrinsics::ctlz32;
//use core::cmp::expect;

use util::bitv::Bitv;

#[repr(u8)]
enum Node {
    UNUSED = 0,
    USED = 1,
    SPLIT = 2,
    FULL = 3
}

pub trait Allocator {
    fn alloc(&mut self, size: uint) -> (*mut u8, uint);

    fn zero_alloc(&mut self, s: uint) -> (*mut u8, uint) {
        let (ptr, size) = self.alloc(s);
        unsafe { set_memory(ptr, 0, size); }
        (ptr, size)
    }

    fn realloc(&mut self, src: *mut u8, size: uint) -> (*mut u8, uint) {
        self.free(src);
        let (ptr, sz) = self.alloc(size);
        unsafe { copy_memory(ptr, src as *const u8, sz); }
        (ptr, sz)
    }

    fn free(&mut self, ptr: *mut u8);
}

pub struct BuddyAlloc {
    pub order: uint,
    pub tree: Bitv
}

pub struct Alloc {
    pub parent: BuddyAlloc,
    pub base: *mut u8,
    pub el_size: uint
}

impl BuddyAlloc {
    pub fn new(order: uint, storage: Bitv) -> BuddyAlloc {
        storage.clear(1 << (order + 1));
        BuddyAlloc { order: order, tree: storage }
    }

    #[inline]
    fn offset(&self, index: uint, level: uint) -> uint {
        (index + 1 - (1 << self.order >> level)) << level
    }

    fn alloc(&mut self, mut size: uint) -> (uint, uint) {
        if size == 0 {
            size = 1;
        }
        // smallest power of 2 >= size
        let lg2_size = 32 - unsafe { ctlz32(size as u32 - 1) } as uint;

        let mut index = 0; // points to current tree node
        let mut level = self.order; // current height

        loop {
            match (self.get(index), level == lg2_size) {
                (UNUSED, true) => {
                    // Found appropriate unused node
                    self.set(index, USED); // use

                    let mut parent = index;
                    loop {
                        let buddy = parent - 1 + (parent & 1) * 2;
                        match self.get(buddy) {
                            USED | FULL if parent > 0 => {
                                parent = (parent + 1) / 2 - 1;
                                self.set(parent, FULL);
                            }
                            _ => break
                        }
                    }
                    return (
                        self.offset(index, level),
                        1 << lg2_size
                    );
                }
                (UNUSED, false) => {
                    // This large node is unused, split it!
                    self.set(index, SPLIT);
                    self.set(index*2 + 1, UNUSED);
                    self.set(index*2 + 2, UNUSED);
                    index = index * 2 + 1; // left child
                    level -= 1;
                }
                (SPLIT, false) => {
                    // Traverse children
                    index = index * 2 + 1; // left child
                    level -= 1;
                }
                _ => loop {
                    // Go either right or back up
                    if index & 1 == 1 {
                        // right sibling
                        index += 1;
                        break;
                    }

                    // go up by one level
                    level += 1;

                    if index == 0 {
                        // out of memory -- back at tree's root after traversal
                        return (0, 0);
                    }

                    index = (index + 1) / 2 - 1; // parent
                }
            }
        }
    }

    fn free(&mut self, offset: uint) {
        let mut length = 1 << self.order;
        let mut left = 0;
        let mut index = 0;

        loop {
            match self.get(index) {
                UNUSED => return,
                USED => loop {
                    if index == 0 {
                        self.set(0, UNUSED);
                        return;
                    }

                    let buddy = index - 1 + (index & 1) * 2;
                    match self.get(buddy) {
                        UNUSED => {}
                        _ => {
                            self.set(index, UNUSED);
                            loop {
                                let parent = (index + 1) / 2 - 1; // parent
                                match self.get(parent) {
                                    FULL if index > 0 => {
                                        self.set(parent, SPLIT);
                                    }
                                    _ => return
                                }
                                index = parent;
                            }
                        }
                    }
                    index = (index + 1) / 2 - 1; // parent
                },
                _ => {
                    length /= 2;
                    if offset < left + length {
                        index = index * 2 + 1; // left child
                    }
                    else {
                        left += length;
                        index = index * 2 + 2; // right child
                    }
                }
            }
        }
    }

    fn get(&self, i: uint) -> Node {
        unsafe {
            transmute(self.tree.get(i))
        }
    }

    fn set(&self, i: uint, x: Node) {
        self.tree.set(i, x as u8);
    }
}

impl Allocator for Alloc {
    fn alloc(&mut self, size: uint) -> (*mut u8, uint) {
        let (offset, size) = self.parent.alloc(size);
        unsafe {
            return (
                self.base.offset((offset << self.el_size) as int),
                size << self.el_size
            )
        }
    }

    fn free(&mut self, ptr: *mut u8) {
        let length = 1 << self.parent.order << self.el_size;

        unsafe {
            if ptr < self.base || ptr >= self.base.offset(length) {
                return;
            }
        }

        let offset = (ptr as uint - self.base as uint) >> self.el_size;
        self.parent.free(offset);
    }
}

impl Alloc {
    pub fn new(parent: BuddyAlloc, base: *mut u8, el_size: uint) -> Alloc {
        Alloc { parent: parent, base: base, el_size: el_size }
    }
}
