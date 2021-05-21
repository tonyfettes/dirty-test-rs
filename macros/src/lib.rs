extern crate proc_macro;
extern crate darling;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod panic;
mod test_case;

use proc_macro2::TokenStream;
use crate::panic::micro_panic_relay_impl;
use crate::panic::micro_panic_receiver_impl;
use crate::test_case::micro_test_case_impl;

/// Test function marker attribute of crate [micro_test](index.html)
///
/// # Usage
///
/// This attribute should be used just like `#[test_case]` attribute.
///
/// ```
/// # #![feature(custom_test_frameworks)]
/// # #![test_runner(test_runner)]
/// # use micro_test::micro_assert_eq;
/// # use micro_test::micro_test_case;
/// #[micro_test_case]
/// fn test_function() { }
/// # fn main() { }
/// ```
///
/// Additional arguments could be passed to this attribute, specifying the
/// testing target and the feature of the target to be tested by this function.
/// ```
/// # #![feature(custom_test_frameworks)]
/// # #![test_runner(test_runner)]
/// # use micro_test::micro_assert_eq;
/// # use micro_test::micro_test_case;
/// #[micro_test_case(target = "test target", feature = "feature tested")]
/// fn another_test_function() { }
/// # fn main() { }
/// ```
///
/// # Explanations
///
/// This procedural macro turns the test function into
/// ```
/// #[test_case]
/// fn test_function() {
///     micro_test::call_user_processor_prepare(Metadata {
///         target: "test target",
///         feature: "feature tested"
///     });
///     { /* original function body */ }
///     micro_test::call_user_processor_settle(Ok(()));
/// }
/// ```
#[proc_macro_attribute]
pub fn micro_test_case(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let output = micro_test_case_impl(attr_args, TokenStream::from(item));
    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn micro_panic_relay(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    proc_macro::TokenStream::from(micro_panic_relay_impl(item_fn))
}

#[proc_macro_attribute]
pub fn micro_panic_receiver(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    proc_macro::TokenStream::from(micro_panic_receiver_impl(item_fn))
}
