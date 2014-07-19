
#![crate_name = "initram"]
#![no_std]
#![no_main]
#![feature(asm, lang_items)]

#[no_mangle]
#[cfg(target_arch = "x86")]
pub unsafe extern "C" fn _start() {
	loop {
		asm!("hlt" :::: "volatile", "intel");
	}
}

#[no_mangle]
#[cfg(target_arch = "arm")]
pub unsafe extern "C" fn _start() {
	loop {
		asm!("" :::: "volatile", "intel");
	}
}
