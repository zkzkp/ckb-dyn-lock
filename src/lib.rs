#![no_std]

#[cfg(not(feature = "lock_binary"))]
extern crate alloc;

#[cfg(any(feature = "lock_binary", feature = "test_tool"))]
extern crate std;

#[cfg(feature = "dynamic_loading")]
pub mod dynamic_loading;
pub mod locks;
#[cfg(feature = "test_tool")]
pub mod test_tool;

#[cfg(feature = "dynamic_loading")]
pub use dynamic_loading::*;
