use crate::test;
use crate::backtrace;

pub struct Reporter {
    pub metadata: Option<fn(test::Metadata)>,
    pub result: Option<fn(test::Result)>,
    pub call_stack: Option<fn(backtrace::CallStack)>,
}

#[cfg(feature = "spin_once")]
static REPORTER: spin::Once<&Reporter> = spin::Once::new();

#[cfg(feature = "spin_once")]
pub fn report_metadata(metadata: test::Metadata) {
    const NOT_INITIALIZED_ERROR: &'static str = "metadata reporter has not been initialized";
    match REPORTER.get() {
        Some(report) => match report.metadata {
            Some(f) => f(metadata),
            None => panic!("{}", NOT_INITIALIZED_ERROR),
        }
        None => panic!("{}", NOT_INITIALIZED_ERROR),
    }
}

#[cfg(feature = "spin_once")]
pub fn report_result(result: test::Result) {
    const NOT_INITIALIZED_ERROR: &'static str = "result reporter has not been initialized";
    match REPORTER.get() {
        Some(report) => match report.result {
            Some(f) => f(result),
            None => panic!("{}", NOT_INITIALIZED_ERROR),
        },
        None => panic!("{}", NOT_INITIALIZED_ERROR),
    }
}

#[cfg(feature = "spin_once")]
pub fn report_call_stack(call_stack: backtrace::CallStack) {
    const NOT_INITIALIZED_ERROR: &'static str = "call stack reporter has not been initialized";
    match REPORTER.get() {
        Some(report) => match report.call_stack {
            Some(f) => f(call_stack),
            None => panic!("{}", NOT_INITIALIZED_ERROR),
        },
        None => panic!("{}", NOT_INITIALIZED_ERROR),
    }
}

#[cfg(feature = "spin_once")]
/// Sets the global test metadata and result processor
///
/// This function may only be called once in the lifetime of a program. Any test
/// function executed prior to `set_reporter` will panic.
///
/// # Panics
///
/// This function will panic on its second call.
///
/// # Availability
///
/// This method is available on target where `spin` crate could work. If not,
/// feature `racy` should be turned on and [`set_reporter_racy`] will be
/// available then.
///
/// # Examples
///
/// ```rust
/// use micro_test::test::Metadata;
/// use micro_test::report::{Reporter, set_reporter};
/// use micro_test::test::Result as TestResult;
/// fn print_metadata(_metadata: Metadata) { }
/// fn print_result(_result: TestResult) { }
/// fn main() {
///     set_reporter(&Reporter {
///         metadata: Some(print_metadata),
///         result: Some(print_result),
///         call_stack: None,
///     });
/// }
/// ```
///
/// If `set_reporter` is called multiple times, it will panic.
/// ```rust,should_panic
/// # use micro_test::test::Metadata;
/// # use micro_test::report::{Reporter, set_reporter};
/// # use micro_test::test::Result as TestResult;
/// # fn print_metadata(_metadata: Metadata) { }
/// # fn print_result(_result: TestResult) { }
/// # fn main() {
/// static REPORTER: &Reporter = &Reporter {
///     metadata: Some(print_metadata),
///     result: Some(print_result),
///     call_stack: None,
/// };
/// set_reporter(&REPORTER);
/// set_reporter(&REPORTER);
/// # }
/// ```
///
/// [`set_reporter_racy`]: fn.set_reporter_racy.html
pub fn set_reporter(reporter: &'static Reporter) {
    if REPORTER.is_completed() {
        panic!("reporter has already been initialized");
    } else {
        REPORTER.call_once(|| reporter);
    }
}

#[cfg(feature = "racy")]
static mut REPORTER: &Reporter = &Reporter {
    metadata: None,
    result: None,
    call_stack: None,
};

#[cfg(feature = "racy")]
/// A thread-unsafe version of [`set_reporter`]
///
/// This function is available unless you disable the default feature of this
/// crate, which includes feature `spin_once` feature only for now.
///
/// # Safety
///
/// This function is only safe to call when no other processor setting functions
/// are stilling initializing the processor.
///
/// [`set_reporter`]: fn.set_reporter.html
pub unsafe fn set_reporter_racy(reporter: &'static Reporter) {
    REPORTER_RACY = reporter;
}

#[cfg(feature = "racy")]
#[doc(hidden)]
pub fn report_metadata(metadata: test::Metadata) {
    const NOT_INITIALIZED_ERROR: &'static str = "metadata reporter has not been initialized";
    unsafe {
        if let Some(f) = REPORTER_RACY.metadata {
            f(metadata);
        } else {
            panic!("{}", NOT_INITIALIZED_ERROR);
        }
    }
}

#[cfg(feature = "racy")]
#[doc(hidden)]
pub fn report_result(result: test::Result) {
    const NOT_INITIALIZED_ERROR: &'static str = "result reporter has not been initialized";
    unsafe {
        if let Some(f) = REPORTER_RACY.result {
            f(result);
        } else {
            panic!("{}", NOT_INITIALIZED_ERROR);
        }
    }
}

#[cfg(feature = "racy")]
#[doc(hidden)]
pub fn report_call_stack(call_stack: backtrace::CallStack) {
    const NOT_INITIALIZED_ERROR: &'static str = "call stack reporter has not been initialized";
    unsafe {
        if let Some(f) = REPORTER_RACY.call_stack {
            f(call_stack);
        } else {
            panic!("{}", NOT_INITIALIZED_ERROR);
        }
    }
}
