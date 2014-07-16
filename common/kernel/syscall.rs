use core::intrinsics::transmute;

#[repr(uint)]
enum Syscall {
	Read = 0,
    Write = 1,

    Unused = 1<<16
}

/// Passes arguments and control to appropriate syscall handler
/// x86 abi 	=> eax(num), argv(edi, esi, edx, ecx, _, _)
/// amd64 abi	=> rax(num), argv(rdi, rsi, rdx, r10, r9, r8)
pub unsafe fn syscall(num: uint, arg0: int, arg1: int, arg2: int, arg3: int, arg4: int, arg5: int) -> int {
	let call: Syscall = transmute(num);
	match call {
		Read	=> -1,
		Write	=> sys::write(arg0 as uint, arg1 as *const u8, arg2 as uint),
		_ 		=> -1
	}
}

mod sys {
	/// Write buffer to file descriptor
	/// 1 = stdout
	pub fn write(fd: uint, buf: *const u8, count: uint) -> int {
		use core::clone::Clone;
		use core::slice;
		use core::slice::raw::buf_as_slice;
		use core::str::from_utf8;
		use core::option::{Option, Some, None};

		let bytes: &[u8] = b"Hello "; //buf_as_slice(buf, count, |&sl| sl);
		if fd == 1 {
			let opt_msg: Option<&str> = from_utf8(bytes);
			match opt_msg {
				Some(msg)	=> {println!("{0}", msg);}
				None		=> {}
			}
		}
		count as int
	}
}
