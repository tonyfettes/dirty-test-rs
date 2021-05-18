#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

fn add_by_one(num: usize) -> usize {
    num + 1
}

fn print_result(result: micro_test::Result) {
    match result {
        Ok(metadata) => match metadata.feature {
            Some(feature) => {
                println!("test {} ({}) success!", metadata.target, feature);
            }
            None => {
                println!("test {} success!", metadata.target);
            }
        },
        Err(e) => match e.metadata.feature {
            Some(feature) => {
                println!(
                    "test {} ({}) FAILED: {}!",
                    e.metadata.target, feature, e.cause
                );
            }
            None => {
                println!("test {} FAILED: {}!", e.metadata.target, e.cause);
            }
        },
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    micro_test::set_result_processor(print_result);
    for test in tests {
        test();
    }
}

mod tests {
    use super::*;
    use micro_test::micro_test_case;
    use micro_test::micro_assert_eq;

    #[micro_test_case(target = "add_by_one", feature = "return value")]
    pub fn test_add_by_one() {
        let num: usize = 1;
        micro_assert_eq!(
            num + 1,
            add_by_one(num)
        );
    }
}
