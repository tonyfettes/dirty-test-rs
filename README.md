# A Dirty and Minimal Rust Test Framework

## Usage

Mark you test function with `micro_test::micro_test_case`.
```rust
#[micro_test_case(
    target = "your test target",
    feature = "tested feature for target"
)]
fn target_feature_test() {
    micro_assert!(target.feature(), 1, "target.feature() doesn't return 1");
    ...
}
```

If you are lazy, you could try feature `replace_assert` which will search for
top level `assert!` in test function and replace it by `micro_assert!`.

Then set a test result processor before conducting any test.
```rust
fn your_result_processor(result: TestResult) {
    match result {
        Ok(metadata) => {
            println!("test {} ({}) success!", metadata.target, metadata.feature);
        },
        Err(e) => {
            println!("test {} ({}) FAILED!: {}", e.metadata.target, e.metadata.feature, e.cause);
        },
    }
}

fn test_runner(tests: &[&dyn Fn()]) {
    dmtest::set_result_processor(your_result_processor);
    for test in tests {
        test();
    }
}
```
