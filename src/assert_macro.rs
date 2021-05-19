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
/// # use micro_test::micro_assert;
/// micro_assert!(1 + 1 == 2, "Math is broken.");
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
            false => {
                $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                    cause: ::core::format_args!("assertion failed: `{}`", ::core::stringify!($test_expr)),
                }));
                return;
            }
        }
    };
    ($test_expr:expr, $($arg:tt)+) => {
        match $test_expr {
            true => {},
            false => {
                $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                    cause: ::core::format_args!($($arg)+),
                }));
                return;
            }
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
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        cause: ::core::format_args!(
                            r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`"#,
                            left_val, right_val
                        ),
                    }));
                    return;
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        cause: ::core::format_args!(
                            r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`: {}"#,
                            left_val, right_val, $($arg)+
                        ),
                    }));
                    return;
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
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        cause: ::core::format_args!(
                            r#"assertion failed: `(left != right)`
 left: `{:?}`,
right: `{:?}`"#,
                            left_val, right_val
                        ),
                    }));
                    return;
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        cause: ::core::format_args!(
                            r#"assertion failed: `(left != right)`
 left: `{:?}`,
right: `{:?}`: {}"#,
                            left_val, right_val, $($arg)+
                        ),
                    }));
                    return;
                }
            }
        }
    }
}

//$crate::__private_api_process_result(::core::result::Result::Ok($metadata));
