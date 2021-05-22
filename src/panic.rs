pub use micro_test_macros::micro_panic_relay;
pub use micro_test_macros::micro_panic_receiver;

pub struct PanicInfo<'a> {
    pub message: Option<&'a core::fmt::Arguments<'a>>,
    pub location: &'a core::panic::Location<'a>
}

#[cfg(feature = "spin_once")]
static PANIC_HANDLER: spin::Once<fn(&PanicInfo)> = spin::Once::new();

pub fn set_panic_handler(handler: fn(&PanicInfo)) {
    if PANIC_HANDLER.is_completed() {
        panic!("micro_test panic handler has already been initialized");
    } else {
        PANIC_HANDLER.call_once(|| handler);
    }
}

pub fn handle_panic(panic_info: &PanicInfo) {
    match PANIC_HANDLER.get() {
        Some(panic_handler) => panic_handler(panic_info),
        None => panic!("panic handler has not been initialized"),
    }
}

#[macro_export]
macro_rules! micro_panic {
    ($arg:tt) => {
        {
            $crate::panic::handle_panic(&$crate::panic::PanicInfo {
                message: Some(&format_args!($arg)),
                location: ::core::panic::Location::caller(),
            });
            return ::core::result::Result::Err($crate::backtrace::CallStack::new());
        }
    };
    ($($arg:tt)*) => {
        {
            $crate::panic::handle_panic(&$crate::panic::PanicInfo {
                message: Some(&format_args!($($arg)*)),
                location: ::core::panic::Location::caller(),
            });
            return ::core::result::Result::Err($crate::backtrace::CallStack::new());
        }
    }
}

#[macro_export]
macro_rules! micro_call {
    (relay $target:ident($($arg:expr),* $(,)*)) => {
        {
            match $target($($arg),*) {
                Ok(ret) => ret,
                Err(mut call_stack) => {
                    call_stack.calls.push($crate::backtrace::FuncCall {
                        name: ::core::stringify!($target)
                    });
                    return ::core::result::Result::Err(call_stack);
                }
            }
        }
    };
    (result $target:ident($($arg:expr),* $(,)*)) => {
        $target($($arg),*)
    };
    (unwrap $target:ident($($arg:expr),* $(,)*)) => {
        $target($($arg),*).unwrap()
    };
}
