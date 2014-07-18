// bits of things mined from rust-core

pub mod c_types;
pub mod fail;

#[cfg(target_arch = "x86")]
#[macro_escape]
pub mod macros;
