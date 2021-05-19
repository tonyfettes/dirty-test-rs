//! A dirty, minimal test framework
//!
//! The `micro_test` together with `micro_test_case` crate provide a attribute
//! that could apply to function which needs to be tested. These two crates
//! depend on `core` only, thus they could be used in `no_std` environments.
//!
//! # Usage
//!
//! The basic usage of this crate is through an attribute
//! [`#[micro_test_case]`], which should be used just like [`#[test_case]`]
//! attribute.
//!
//! Users of this crate should set a `result_processor` function to process the
//! test result. If no implementations are given, the crate will panic on the
//! firstly executed test function.
//!
//! ## Examples
//!
//! ```rust
//! #![feature(custom_test_frameworks)]
//! #![test_runner(test_runner)]
//! 
//! fn add_by_one(num: usize) -> usize {
//!     num + 1
//! }
//! 
//! fn print_result(result: micro_test::Result) {
//!     match result {
//!         Ok(metadata) => match metadata.feature {
//!             Some(feature) => {
//!                 println!("test {} ({}) success!", metadata.target, feature);
//!             }
//!             None => {
//!                 println!("test {} success!", metadata.target);
//!             }
//!         },
//!         Err(e) => match e.metadata.feature {
//!             Some(feature) => {
//!                 println!(
//!                     "test {} ({}) FAILED: {}!",
//!                     e.metadata.target, feature, e.cause
//!                 );
//!             }
//!             None => {
//!                 println!("test {} FAILED: {}!", e.metadata.target, e.cause);
//!             }
//!         },
//!     }
//! }
//! 
//! fn test_runner(tests: &[&dyn Fn()]) {
//!     micro_test::set_result_processor(print_result);
//!     for test in tests {
//!         test();
//!     }
//! }
//! 
//! mod tests {
//!     use super::*;
//!     use micro_test::micro_assert_eq;
//!     use micro_test::micro_test_case;
//! 
//!     #[micro_test_case(target = "add_by_one", feature = "return value")]
//!     pub fn test_add_by_one() {
//!         let num: usize = 1;
//!         micro_assert_eq!(
//!             num + 1,
//!             add_by_one(num)
//!         );
//!     }
//! }
//! # fn main() { }
//! ```
//!
//! # Additional Information
//!
//! ## Explanation
//!
//! This test framework works in exactly the same way as [`μtest`] does. Since
//! we call μ- as micro- in metrics, I named this crate as `micro_test`. Though
//! the author of [`μtest`] doesn't recommend to use this approach, I always
//! find myself playing with unstable and nightly features of rust when dealing
//! with bare-metal and needs a macro to simplify some repetitive code in
//! testing.
//!
//! In a function needs to be tested, users should mark it with
//! `#[micro_test_case]` attribute, and this attribute procedural macro will
//! replace every top level call to macro `assert!` (with feature
//! `replace_assert` on) or to macro `micro_assert!` (default behaviour) and
//! inject code than contains calls to the result processing function provided
//! by users.
//!
//! [`μtest`]: https://github.com/japaric/utest
//!
//! ## Comparison
//!
//! Different from `μtest`, this crate doesn't have `#[should_panic]` support,
//! and doesn't intend to replace `panic!` macro invocations. Plus, this crate
//! use trait to let users register their hook functions, instead of using
//! `extern "Rust"`.
//!
//! Different from test framework provided by Rust, this crate doesn't support
//! measuring, benchmarking, filtering, ignoring tests (yet) and mark test with
//! `#[should_panic]`. Measuring and Ignoring tests is planned to be supported.
//!

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate micro_test_case;

#[macro_use]
mod assert_macro;
mod result;

pub use micro_test_case::micro_test_case;
pub use result::Error;
pub use result::Metadata;
pub use result::Result;

static mut RESULT_PROCESSOR: fn(Result) = |_| { };

#[doc(hidden)]
struct VacuumProcessor;

pub trait Process {
    fn prepare(&self, metadata: Metadata);
    fn process(&self, result: Result);
}

impl Process for VacuumProcessor {
    fn prepare(&self, metadata: Metadata) { }
    fn process(&self, result: Result) { }
}

pub fn set_result_processor(processor: fn(Result)) {
    unsafe {
        RESULT_PROCESSOR = processor;
    }
}

#[doc(hidden)]
pub fn __private_api_process_result(result: Result) {
    unsafe {
        RESULT_PROCESSOR(result);
    }
}
