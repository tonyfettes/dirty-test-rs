#![no_std]

extern crate micro_test_case;

#[macro_use]
mod result;

pub use micro_test_case::micro_test_case;
pub use result::Error;
pub use result::Metadata;
pub use result::TestResult;

static mut RESULT_PROCESSOR: fn(TestResult) = |_| {
    unimplemented!("Result processor is not set");
};

pub fn set_result_processor(processor: fn(TestResult)) {
    unsafe {
        RESULT_PROCESSOR = processor;
    }
}

pub fn __private_api_process_result(result: TestResult) {
    unsafe {
        RESULT_PROCESSOR(result);
    }
}
