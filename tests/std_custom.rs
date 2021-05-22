#![feature(custom_test_frameworks)]
#![feature(trace_macros)]
#![test_runner(test_runner)]

fn add_by_one(num: usize) -> usize {
    num + 1
}

fn report_metadata(metadata: &micro_test::test::Metadata) {
    match metadata.feature {
        Some(feature) => {
            print!("test {} ({}) ... ", metadata.target, feature);
        }
        None => {
            print!("test {} ... ", metadata.target);
        }
    }
}

fn panic_handler(info: &micro_test::panic::PanicInfo) {
    println!("FAILED: {}", info);
}

fn test_runner(tests: &[&dyn Fn() -> core::result::Result<(), micro_test::backtrace::CallStack>]) {
    micro_test::test::set_metadata_reporter(report_metadata);
    micro_test::panic::set_panic_handler(panic_handler);
    println!(r#"
running {} tests"#, tests.len());
    for test in tests {
        match test() {
            Ok(_) => println!("ok"),
            Err(call_stack) => {
                for (i, func_call) in call_stack.calls.iter().enumerate() {
                    println!("{}: {}", i, func_call.name);
                }
            }
        }
    }
    println!();
}

mod tests {
    use super::*;
    use micro_test::test::micro_test_case as micro;
    use micro_test::micro_assert_eq;

    #[micro]
    pub fn test_add_by_one_empty() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }

    #[micro(target = "add_by_one", feature = "return value")]
    pub fn test_add_by_one_target_feature() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }

    #[micro(path = true, target = "add_by_one_with_path", feature = "return value")]
    pub fn test_add_by_one_path_target_feature() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }

    #[micro(path = false, target = "add_by_one_without_path", feature = "return value")]
    pub fn test_add_by_one_no_path_target_feature() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }
}
