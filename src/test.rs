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
//! use micro_test::test::Metadata;
//! use micro_test::test::Result as TestResult;
//!
//! fn add_by_one(num: usize) -> usize {
//!     num + 1
//! }
//!
//! fn print_metadata(metadata: Metadata) {
//!     match metadata.feature {
//!         Some(feature) => {
//!             println!("testing {} ({}) ...", metadata.target, feature);
//!         }
//!         None => {
//!             println!("testing {} ...", metadata.target);
//!         }
//!     }
//! }
//!
//! fn print_result(result: TestResult) {
//!     match result {
//!         Ok(_) => println!("ok"),
//!         Err(e) => println!("FAILED: {}", e),
//!     }
//! }
//!
//! fn test_runner(tests: &[&dyn Fn()]) {
//!     micro_test::report::set_reporter(&micro_test::report::Reporter {
//!         metadata: Some(print_metadata),
//!         result: Some(print_result),
//!         call_stack: None,
//!     });
//!     for test in tests {
//!         test();
//!     }
//! }
//!
//! mod tests {
//!     use super::*;
//!     use micro_test::micro_assert_eq;
//!     use micro_test::test::micro_test_case;
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
//!
//! fn main() { }
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

pub use micro_test_macros::micro_test_case;
use core::fmt::Result as FmtResult;
use core::fmt::{Debug, Display, Formatter};

pub use crate::panic::PanicInfo as Error;

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
/// # use micro_test::test::Metadata;
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

impl Metadata {
    pub const fn new() -> Self {
        Self {
            target: "",
            feature: None,
        }
    }
}

#[cfg(feature = "spin_once")]
static METADATA_HANDLER: spin::Once<fn(&Metadata)> = spin::Once::new();

pub fn set_metadata_reporter(reporter: fn(&Metadata)) {
    if METADATA_HANDLER.is_completed() {
        panic!("micro_test metadata reporter has already been initialized");
    } else {
        METADATA_HANDLER.call_once(|| reporter);
    }
}

pub fn report_metadata(metadata: &Metadata) {
    match METADATA_HANDLER.get() {
        Some(metadata_handler) => metadata_handler(metadata),
        None => panic!("metadata reporter has not been initialized"),
    }
}

// The error type contains cause in the form of [format
// arguments](https://doc.rust-lang.org/core/fmt/struct.Arguments.html)
//#[derive(Clone, Debug)]
//pub struct Error<'a> {
//    pub cause: core::fmt::Arguments<'a>,
//}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.message {
            Some(msg) => f.write_fmt(format_args!("{}: {}", msg, self.location)),
            None => f.write_fmt(format_args!("{}", self.location)),
        }
    }
}

/// The type of result of one single test.
pub type Result<'a> = ::core::result::Result<(), Error<'a>>;

/// The assertion macro used in micro_test
///
/// This macro will call result processing function set by users when assertion
/// failed and returns, and therefore is NOT HYGIENIC. There is no point to use
/// this macro outside test function marked with `#[micro_test_case]`, and it
/// will turn into a common `assert!`.
///
/// # Example
///
/// ```rust
/// # #![feature(custom_test_frameworks)]
/// # use micro_test::micro_assert;
/// # use micro_test::test::micro_test_case;
/// # #[micro_test_case]
/// # fn test() {
/// micro_assert!(1 + 1 == 2, "Math is broken.");
/// # }
/// ```
///
/// # Explanations
///
/// This macro does these things:
///
/// 1. Evaluate assertion expression.
/// 2. Does nothing is the expression returns true, or call user's result
///    processing function.
/// 3. If the assertion failed, return current test function.
///
/// And the implement could be seen as
/// ```no_run
/// macro_rules! micro_assert {
///     ($test_expr:expr, $($arg:tt)*) => {
///         if $test_expr {
///             // do nothing
///         } else {
///             user_process_result( ... );
///             return;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! micro_assert {
    ($test_expr:expr $(,)?) => {
        match $test_expr {
            true => {},
            false => $crate::micro_panic!("assertion failed: `{}`", ::core::stringify!($test_expr)),
        }
    };
    ($test_expr:expr, $($arg:tt)+) => {
        match $test_expr {
            true => {},
            false => $crate::micro_panic!($($arg)+),
        }
    }
}

/// Asserts two expression is equal.
///
/// See [`micro_assert](macro.micro_assert.html).
#[macro_export]
macro_rules! micro_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::micro_panic!(r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`"#, left_val, right_val);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::micro_panic!(r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`: {}"#, left_val, right_val, $($arg)+);
                }
            }
        }
    }
}

/// Asserts two expression is not equal.
///
/// See [`micro_assert](macro.micro_assert.html).
#[macro_export]
macro_rules! micro_assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::micro_panic!(r#"assertion failed: `(left != right)`
 left: `{:?}`,
right: `{:?}`"#, left_val, right_val);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::micro_panic!(relay r#"assertion failed: `(left != right)`
 left: `{:?}`,
right: `{:?}`: {}"#, left_val, right_val, $($arg)+);
                }
            }
        }
    }
}
