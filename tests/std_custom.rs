#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use micro_test::report::Reporter;

fn add_by_one(num: usize) -> usize {
    num + 1
}

fn report_metadata(metadata: micro_test::test::Metadata) {
    match metadata.feature {
        Some(feature) => {
            print!("test {} ({}) ... ", metadata.target, feature);
        }
        None => {
            print!("test {} ... ", metadata.target);
        }
    }
}

fn report_result(result: micro_test::test::Result) {
    match result {
        Ok(_) => println!("ok"),
        Err(e) => println!("FAILED: {}", e),
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    micro_test::report::set_reporter(&Reporter {
        metadata: Some(report_metadata),
        result: Some(report_result),
        call_stack: None,
    });
    println!(r#"
running {} tests"#, tests.len());
    for test in tests {
        test();
    }
}

mod tests {
    use super::*;
    use micro_test::test::micro_test_case;
    use micro_test::micro_assert_eq;

    #[micro_test_case]
    pub fn test_add_by_one_empty() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }

    #[micro_test_case(target = "add_by_one", feature = "return value")]
    pub fn test_add_by_one_target_feature() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }

    #[micro_test_case(path = true, target = "add_by_one_with_path", feature = "return value")]
    pub fn test_add_by_one_path_target_feature() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }

    #[micro_test_case(path = false, target = "add_by_one_without_path", feature = "return value")]
    pub fn test_add_by_one_no_path_target_feature() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }
}
