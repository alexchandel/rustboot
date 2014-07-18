/* rt.rs
 */

use core::intrinsics::{ctlz32, cttz32};
use core::mem::{transmute, size_of};

mod detail {
    extern {
        #[link_name = "llvm.debugtrap"]
        pub fn breakpoint();
    }
}

#[no_mangle]
pub fn breakpoint() {
    unsafe { detail::breakpoint() }
}
