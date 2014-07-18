pub mod interrupt;
pub mod mmu;

pub fn init() {
    unsafe {
        mmu::init();
    }
}

pub fn info() {
}
