#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

fn add_by_one(num: usize) -> usize {
    num + 1
}

struct SimpleTestProcessor;

impl micro_test::Process for SimpleTestProcessor {
    fn prepare(&self, metadata: micro_test::Metadata) {
        match metadata.feature {
            Some(feature) => {
                println!("testing {} ({}) ...", metadata.target, feature);
            }
            None => {
                println!("testing {} ...", metadata.target);
            }
        }
    }
    fn settle(&self, result: micro_test::Result) {
        match result {
            Ok(_) => println!("ok"),
            Err(e) => println!("FAILED: {}", e),
        }
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    static TEST_PROCESSOR: SimpleTestProcessor = SimpleTestProcessor;
    micro_test::set_processor(&TEST_PROCESSOR);
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
