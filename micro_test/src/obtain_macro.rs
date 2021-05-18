#[macro_export]
macro_rules! obtain_result_from_assert {
    ($test_metadata:expr, $test_expr:expr $(,)?) => {
        match $test_expr {
            true => ::core::result::Result::Ok($test_metadata),
            false => ::core::result::Result::Err($crate::tests::TestError {
                metadata: $test_metadata,
                cause: ::alloc::format!("assertion failed: `{}`", ::core::stringify!($test_expr)),
            })
        }
    };
    ($test_metadata:expr, $test_expr:expr, $format_args:expr) => {
        match $test_expr {
            true => ::core::result::Result::Ok($test_metadata),
            false => ::core::result::Result::Err($crate::tests::TestError {
                metadata: $test_metadata,
                cause: ::alloc::format!("{}", $format_args),
            })
        }
    }
}

#[macro_export]
macro_rules! result_from_assert_eq {
    ($test_metadata:expr, $left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::result::Result::Err($crate::tests::TestError {
                        metadata: $test_metadata,
                        cause: ::alloc::format!(
                            r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`"#,
                            left_val, right_val
                        ),
                    })
                } else {
                    ::core::result::Result::Ok($test_metadata)
                }
            }
        }
    };
    ($test_metadata:expr, $left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    ::core::result::Result::Err($crate::tests::TestError {
                        metadata: $test_metadata,
                        cause: ::alloc::format!(
                            r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`: {}"#,
                            left_val, right_val, $($arg)+
                        ),
                    })
                } else {
                    ::core::result::Result::Ok($test_metadata)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! result_from_assert_ne {
    ($test_metadata:expr, $test_expr_a:expr, $test_expr_b:expr) => {
        match $test_expr_a != $test_expr_b {
            true => ::core::result::Result::Ok($test_metadata),
            false => ::core::result::Result::Err($crate::tests::TestError {
                metadata: $test_metadata,
                cause: ::alloc::format!(
                    "assertion failed: `({} != {})`\n left: `{:?}`,\nright: `{:?}`",
                    ::core::stringify!($test_expr), ::core::stringify!($test_expr),
                    $test_expr_a, $test_expr_b),
            })
        }
    };
    ($test_metadata:expr, $test_expr_a:expr, $test_expr_b:expr, $format_args:expr) => {
        match $test_expr_a == $test_expr_b {
            true => ::core::result::Result::Ok($test_metadata),
            false => ::core::result::Result::Err($crate::tests::TestError {
                metadata: $test_metadata,
                cause: ::alloc::format!("{}", $format_args),
            })
        }
    }
}

pub fn merge<'a>(results: &'a [TestResult]) -> TestResult<'a> {
    let self_metadata = Metadata {
        target: core::stringify!(TestResult),
        ty: TestType::Function,
        feature: "merge",
    };
    let mut final_metadata: Option<Metadata> = None;
    for result in results {
        match result {
            Ok(metadata) => {
                match final_metadata {
                    Some(_) => {},
                    None => { final_metadata = Some(*metadata); },
                }
                drop(result);
            },
            Err(error) => {
                let ret = Err(error.clone());
                drop(result);
                return ret;
            }
        }
    }
    match final_metadata {
        Some(final_metadata) => {
            return Ok(final_metadata);
        },
        None => {
            return Err(Error {
                metadata: self_metadata,
                cause: ::alloc::format!("Error getting test metadata"),
            });
        }
    }
}
