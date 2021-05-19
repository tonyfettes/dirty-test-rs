//! A dirty, minimal test framework
//!
//! The `micro_test` together with `micro_test_case` crate provide a attribute
//! that could apply to function which needs to be tested. These two crates
//! depend on `core` only, thus they could be used in `no_std` environments.
//!
//! # Usage
//!
//! The basic usage of this crate is through an attribute
//! [`micro_test_case`].
//!
//! Users of this crate should set a _processor_ with trait [`Process`] to process the
//! test metadata and result. If no implementations are given, the crate will panic on the
//! firstly executed test function.
//!
//! [`Process`]: trait.Process.html
//! [`micro_test_case`]: attr.micro_test_case.html
//!
//! ## Examples
//!
//! ```rust
//! #![feature(custom_test_frameworks)]
//! #![test_runner(test_runner)]
//!
//! use micro_test::Metadata;
//! use micro_test::Result as TestResult;
//!
//! fn add_by_one(num: usize) -> usize {
//!     num + 1
//! }
//!
//! struct SimpleTestProcessor;
//!
//! impl micro_test::Process for SimpleTestProcessor {
//!     fn prepare(&self, metadata: Metadata) {
//!         match metadata.feature {
//!             Some(feature) => {
//!                 println!("testing {} ({}) ...", metadata.target, feature);
//!             }
//!             None => {
//!                 println!("testing {} ...", metadata.target);
//!             }
//!         }
//!     }
//!     fn settle(&self, result: TestResult) {
//!         match result {
//!             Ok(_) => println!("ok"),
//!             Err(e) => println!("FAILED: {}", e),
//!         }
//!     }
//! }
//!
//! fn test_runner(tests: &[&dyn Fn()]) {
//!     static TEST_PROCESSOR: SimpleTestProcessor = SimpleTestProcessor;
//!     micro_test::set_processor(&TEST_PROCESSOR);
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
//! # Explanations
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
//! # Comparison
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
#![cfg_attr(feature = "once_cell", feature(once_cell))]

#[cfg(all(feature = "spin_once", feature = "racy"))]
compile_error!("features `micro_test/spin_once` and `micro_test/racy` are mutually exclusive");

extern crate micro_test_case;

#[macro_use]
mod assert_macro;

pub use micro_test_case::micro_test_case;

use core::fmt::Result as FmtResult;
use core::fmt::{Debug, Display, Formatter};

/// Metadata about a test
///
/// # Use
///
/// `Metadata` structs are created at the beginning of test function. They are
/// send to [`prepare`] method of a user-defined variable with the trait
/// [`Process`] to be processed.
///
/// # Examples
///
/// ```rust
/// # use micro_test::Metadata;
/// fn print_metadata(metadata: Metadata) {
///     match metadata.feature {
///         Some(feature) => {
///             println!("testing target {} with feature {}", metadata.target, feature);
///         },
///         None => {
///             println!("testing target {}", metadata.target);
///         }
///     }
/// }
///
/// # fn main() {
/// print_metadata(Metadata {
///     target: "crate::print_metadata",
///     feature: Some("metadata printing"),
/// });
/// # }
/// ```
///
/// [`prepare`]: trait.Process.html#tymethod.write_str
/// [`Process`]: trait.Process.html
#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    pub target: &'static str,
    pub feature: Option<&'static str>,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.feature {
            Some(feature) => f.write_fmt(format_args!("{} ({})", self.target, feature)),
            None => f.write_fmt(format_args!("{}", self.target)),
        }
    }
}

/// The error type contains cause in the form of [format
/// arguments](https://doc.rust-lang.org/core/fmt/struct.Arguments.html)
#[derive(Clone, Debug)]
pub struct Error<'a> {
    pub cause: core::fmt::Arguments<'a>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("{}", self.cause))
    }
}

/// The type of result of one single test.
pub type Result<'a> = ::core::result::Result<(), Error<'a>>;

#[doc(hidden)]
struct VacuumProcessor;

/// A trait for processing metadata and result, setting up and tearing down the
/// test environments.
pub trait Process: Sync + Send {

    /// Processes metadata of the test and sets up testing environments.
    fn prepare(&self, metadata: Metadata);

    /// Processes result of the test and tears down the testing environments.
    fn settle(&self, result: crate::Result);
}

/// A trait describes how to obtain measure current time and obtain time
/// duration.
pub trait Measure: Sync + Send {

    /// Instant of time. See
    /// [std::time::Instant](https://doc.rust-lang.org/std/time/struct.Instant.html).
    type Instant;

    /// Span of time. See
    /// [std::time::Duration](https://doc.rust-lang.org/std/time/struct.Duration.html).
    type Duration;

    /// Returns an instant corresponding to "now".
    fn now() -> Self::Instant;

    /// Returns an duration between two instant of time, or
    /// [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None)
    /// if the `start` is behind `end`.
    fn duration(start: Self::Instant, end: Self::Instant) -> Option<Self::Duration>;
}

/// A vacuum implementation of trait [`Process`](trait.Process.html)
impl Process for VacuumProcessor {
    fn prepare(&self, _metadata: Metadata) { }
    fn settle(&self, _result: crate::Result) { }
}

#[cfg(feature = "spin_once")]
static PROCESSOR: spin::Once<&dyn Process> = spin::Once::new();

#[cfg(feature = "racy")]
static PROCESSOR: &dyn Process = &VacuumProcessor;

#[cfg(feature = "spin_once")]
/// Sets the global test metadata and result processor
///
/// This function may only be called once in the lifetime of a program. Any test
/// function executed prior to `set_processor` will panic.
///
/// # Panics
///
/// This function will panic on its second call.
///
/// # Availability
///
/// This method is available on target where `spin` crate could work. If not,
/// feature `racy` should be turned on and [`set_processor_racy`] will be
/// available then.
///
/// # Examples
///
/// ```rust
/// use micro_test::{Process, Metadata, set_processor};
/// use micro_test::Result as TestResult;
/// struct ExampleProcessor;
/// impl Process for ExampleProcessor {
///     fn prepare(&self, _metadata: Metadata) { }
///     fn settle(&self, _result: TestResult) { }
/// }
/// fn main() {
///     static EXAMPLE_PROCESSOR: ExampleProcessor = ExampleProcessor;
///     set_processor(&EXAMPLE_PROCESSOR);
/// }
/// ```
///
/// If `set_processor` is called multiple times, it will panic.
/// ```rust,should_panic
/// # use micro_test::{Process, Metadata, set_processor};
/// # use micro_test::Result as TestResult;
/// # struct ExampleProcessor;
/// # impl Process for ExampleProcessor {
/// #     fn prepare(&self, _metadata: Metadata) { }
/// #     fn settle(&self, _result: TestResult) { }
/// # }
/// # fn main() {
/// #     static EXAMPLE_PROCESSOR: ExampleProcessor = ExampleProcessor;
/// set_processor(&EXAMPLE_PROCESSOR);
/// set_processor(&EXAMPLE_PROCESSOR); // panic
/// # }
/// ```
///
/// [`set_processor_racy`]: fn.set_processor_racy.html
pub fn set_processor(processor: &'static dyn Process) {
    if PROCESSOR.is_completed() {
        panic!("metadata and result processor has already been initialized");
    } else {
        PROCESSOR.call_once(|| processor);
    }
}

#[cfg(feature = "racy")]
/// A thread-unsafe version of [`set_processor`]
///
/// This function is available unless you disable the default feature of this
/// crate, which includes feature `spin_once` feature only for now.
///
/// # Safety
///
/// This function is only safe to call when no other processor setting functions
/// are stilling initializing the processor.
///
/// [`set_processor`]: fn.set_processor.html
pub unsafe fn set_processor_racy(processor: &'static dyn Process) {
    PROCESSOR = processor;
}

#[cfg(feature = "spin_once")]
#[doc(hidden)]
#[inline]
pub fn __private_api_process_metadata(metadata: Metadata) {
    match PROCESSOR.get() {
        Some(processor) => processor.prepare(metadata),
        None => panic!("metadata and result processor has not been initialized"),
    }
}

#[cfg(feature = "spin_once")]
#[doc(hidden)]
#[inline]
pub fn __private_api_process_result(result: Result) {
    match PROCESSOR.get() {
        Some(processor) => processor.settle(result),
        None => panic!("metadata and result processor has not been initialized"),
    }
}

#[cfg(feature = "racy")]
#[doc(hidden)]
#[inline]
pub fn __private_api_process_metadata(metadata: Metadata) {
    PROCESSOR.prepare(metadata);
}

#[cfg(feature = "racy")]
#[doc(hidden)]
#[inline]
pub fn __private_api_process_result(result: Result) {
    PROCESSOR.settle(result);
}
