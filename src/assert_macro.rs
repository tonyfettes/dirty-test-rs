/// The assertion macro used in micro_test
///
/// This macro will call result processing function set by users when assertion
/// failed and returns, and therefore is NOT HYGIENIC. There is no point to use
/// this macro outside test function marked with `#[micro_test_case]`, and it
/// will turn into a common `aseert!`.
///
/// # Example
///
/// ```rust
///
/// ```
///
/// # Explanation
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
        assert!($test_expr);
    };
    ($test_expr:expr, $($arg:tt)+) => {
        assert!($test_expr, $($arg)+);
    };
    (metadata $metadata:expr, $test_expr:expr $(,)?) => {
        match $test_expr {
            true => {},
            false => {
                $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                    metadata: $metadata,
                    cause: ::core::format_args!("assertion failed: `{}`", ::core::stringify!($test_expr)),
                }));
                return;
            }
        }
    };
    (metadata $metadata:expr, $test_expr:expr, $($arg:tt)+) => {
        match $test_expr {
            true => {},
            false => {
                $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                    metadata: $metadata,
                    cause: ::core::format_args!($($arg)+),
                }));
                return;
            }
        }
    }
}

#[macro_export]
macro_rules! micro_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        assert_eq!($left, $right);
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        assert_eq!($left, $right, $($arg)+);
    };
    (metadata $metadata:expr, $left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
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
    (metadata $metadata:expr, $left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
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

#[macro_export]
macro_rules! micro_assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        assert_ne!($left, $right);
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        assert_ne!($left, $right, $($arg)+);
    };
    (metadata $metadata:expr, $left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
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
    (metadata $metadata:expr, $left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
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
