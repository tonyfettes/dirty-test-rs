#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![cfg_attr(feature = "once_cell", feature(once_cell))]

#[cfg(all(feature = "spin_once", feature = "racy"))]
compile_error!("features `micro_test/spin_once` and `micro_test/racy` are mutually exclusive");

extern crate micro_test_macros;

pub mod bench;
#[macro_use]
pub mod test;
pub use test::micro_test_case;
pub mod panic;
pub mod backtrace;
pub mod report;
