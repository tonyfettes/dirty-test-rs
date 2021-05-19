extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromMeta)]
struct FullAttrArgs {
    target: String,
    feature: String,
}

#[derive(FromMeta)]
struct TargetAttrArgs {
    target: String,
}

#[derive(FromMeta)]
struct EmptyAttrArgs {}

struct ProcMacroAttrArgs {
    target: Option<String>,
    feature: Option<String>,
}

const METADATA_NAME: &'static str =
    "__micro_test_private_api_metadata_X19yZXRlc3RfcHJpdmF0ZV9hcGlfbWV0YWRhdGEK";
const RESULT_PROCESSOR_NAME: &'static str = "__private_api_process_result";

#[cfg(feature = "replace_assert")]
fn transform_macro(mac: &mut syn::Macro, micro_test_crate: String) {
    assert_eq!(
        mac.path.segments.len(),
        1,
        "#[micro_test_assert] should only be applied to assert* macro invocation: path length: {}",
        mac.path.segments.len()
    );
    let old_string = mac.path.segments.last().unwrap().ident.to_string();
    if old_string.starts_with("assert") {
        let new_string: String = micro_test_crate + "::" + "micro_" + &old_string;
        let new_ts = new_string.parse::<TokenStream>();
        let new_ts = match new_ts {
            Ok(ts) => ts,
            Err(e) => panic!("Failed to parse new_string: {}", e),
        };
        mac.path = syn::parse2::<syn::Path>(new_ts).unwrap();
        let old_tokens = mac.tokens.clone();
        let metadata: syn::Ident =
            syn::parse2(METADATA_NAME.parse::<TokenStream>().unwrap()).unwrap();
        mac.tokens = quote!(metadata #metadata, #old_tokens).into();
    }
}

#[cfg(not(feature = "replace_assert"))]
fn transform_macro(mac: &mut syn::Macro, micro_test_crate: String) {
    let old_string = mac.path.segments.last().unwrap().ident.to_string();
    if old_string.len() == 2 {
        assert_eq!(mac.path.segments.first().unwrap().ident.to_string(), micro_test_crate);
    }
    if old_string.starts_with("micro_assert") {
        let old_tokens = mac.tokens.clone();
        let metadata: syn::Ident =
            syn::parse2(METADATA_NAME.parse::<TokenStream>().unwrap()).unwrap();
        mac.tokens = quote!(metadata #metadata, #old_tokens).into();
    }
}

fn transform_expr(expr: &mut syn::Expr, micro_test_crate: &String) {
    use syn::Expr::*;
    match expr {
        Macro(macro_expr) => transform_macro(&mut macro_expr.mac, micro_test_crate.clone()),
        If(if_expr) => {
            transform_block(&mut if_expr.then_branch, micro_test_crate);
            match if_expr.else_branch.clone() {
                Some((_, mut else_branch_expr)) => {
                    transform_expr(&mut else_branch_expr, micro_test_crate)
                }
                None => (),
            }
        }
        ForLoop(for_loop_expr) => transform_block(&mut for_loop_expr.body, micro_test_crate),
        Loop(loop_expr) => transform_block(&mut loop_expr.body, micro_test_crate),
        While(while_expr) => transform_block(&mut while_expr.body, micro_test_crate),
        Block(block_expr) => transform_block(&mut block_expr.block, micro_test_crate),
        Match(match_expr) => {
            let arms = &mut match_expr.arms;
            for arm in arms {
                transform_expr(&mut *arm.body, micro_test_crate);
            }
        }
        _ => (),
    }
}

fn transform_stmt(stmt: &mut syn::Stmt, micro_test_crate: &String) {
    match stmt {
        syn::Stmt::Expr(expr) => transform_expr(expr, micro_test_crate),
        syn::Stmt::Semi(semi_expr, _) => transform_expr(semi_expr, micro_test_crate),
        _ => (),
    }
}

fn transform_block(block: &mut syn::Block, micro_test_crate: &String) {
    let statements = &mut block.stmts;
    for mut statement in statements {
        transform_stmt(&mut statement, micro_test_crate);
    }
}

#[cfg(any())]
#[proc_macro_attribute]
pub fn micro_test_module(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    unimplemented!("Unimplemented yet");
    let input = syn::parse_macro_input!(item as syn::ItemMod);
    assert!(input.semi.is_none(), "This attribute should not be applied to: mod m;");
    let (brace, mut items) = match input.content {
        Some(content) => content,
        None => panic!("This attribute should not be applied to: mod m;"),
    };
    micro_test_module_impl(&mut items);
    let output = syn::ItemMod {
        attrs: input.attrs,
        vis: input.vis,
        mod_token: input.mod_token,
        ident: input.ident,
        content: Some((brace, items)),
        semi: input.semi,
    };
    quote!(#output).into()
}

#[cfg(any())]
#[proc_macro_attribute]
fn micro_test_module_impl(items: &mut Vec<syn::Item>) {
    for item in items {
        match item {
            syn::Item::Use(item_use) => {
            },
            syn::Item::Fn(item_fn) => {
            },
            _ => {
            }
        }
    }
}

#[proc_macro_attribute]
pub fn micro_test_case(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let orig_attr_args = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let attr_args = match FullAttrArgs::from_list(&orig_attr_args) {
        Ok(v) => ProcMacroAttrArgs {
            target: Some(v.target),
            feature: Some(v.feature),
        },
        Err(_) => match TargetAttrArgs::from_list(&orig_attr_args) {
            Ok(v) => ProcMacroAttrArgs {
                target: Some(v.target),
                feature: None,
            },
            Err(_) => match EmptyAttrArgs::from_list(&orig_attr_args) {
                Ok(_) => ProcMacroAttrArgs {
                    target: None,
                    feature: None,
                },
                Err(e) => {
                    return proc_macro::TokenStream::from(e.write_errors());
                }
            },
        },
    };
    let output = micro_test_case_impl(attr_args, TokenStream::from(item));
    //panic!("OUTPUT_TOKENS: {}", output.to_string());
    proc_macro::TokenStream::from(output)
}

fn micro_test_case_impl(attr_args: ProcMacroAttrArgs, item: TokenStream) -> TokenStream {
    // Get the name of micro_test crate
    let micro_test_crate_string = match proc_macro_crate::crate_name("micro_test") {
        Ok(founded_crate) => match founded_crate {
            proc_macro_crate::FoundCrate::Itself => String::from("micro_test"),
            proc_macro_crate::FoundCrate::Name(name_string) => name_string,
        },
        Err(e) => panic!("Cannot find micro_test crate: {}", e),
    };

    let mut input = syn::parse2::<syn::ItemFn>(item).unwrap();

    // Process the function signature
    let signature = input.sig.clone();
    if signature.asyncness.is_some() {
        panic!("#[micro_test_case] test function should not be async");
    }
    if signature.generics.lt_token.is_some()
        || signature.generics.gt_token.is_some()
        || signature.generics.where_clause.is_some()
    {
        panic!("#[micro_test_case] test function should not have generics");
    }
    match signature.output {
        syn::ReturnType::Default => (),
        syn::ReturnType::Type(_, _) => {
            panic!("#[micro_test_case] test function should not have return type")
        }
    }
    assert_eq!(signature.inputs.len(), 0, "#[micro_test_case] test function should not have inputs");
    let function_name = signature.ident.clone();

    let target = match attr_args.target {
        Some(target) => target,
        None => function_name.to_string(),
    };
    let feature: TokenStream = match attr_args.feature {
        Some(feature) => {
            let some_token = "Some(\"".to_owned() + &feature + "\")";
            some_token.parse().unwrap()
        }
        None => quote!(None).into(),
    };

    // Process the function body
    let mut block = &mut *input.block;
    transform_block(&mut block, &micro_test_crate_string);

    let micro_test_crate = syn::Ident::from_string(&micro_test_crate_string).unwrap();
    let metadata = syn::Ident::from_string(METADATA_NAME).unwrap();
    let result_processor = syn::Ident::from_string(RESULT_PROCESSOR_NAME).unwrap();

    let new_block = quote! {
        {
            const #metadata: #micro_test_crate::Metadata = #micro_test_crate::Metadata {
                target: #target,
                feature: #feature,
            };
            #block
            #micro_test_crate::#result_processor(::core::result::Result::Ok(#metadata));
        }
    };
    input.block = Box::new(syn::parse2::<syn::Block>(new_block).unwrap());
    quote! {
        #[test_case]
        #input
    }
}

#[cfg(any())]
#[proc_macro_attribute]
pub fn micro_test_assert(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let micro_test_crate = match proc_macro_crate::crate_name("micro_test") {
        Ok(founded_crate) => match founded_crate {
            proc_macro_crate::FoundCrate::Itself => {
                panic!("The name of this proc_macro crate should be micro_test_macro")
            }
            proc_macro_crate::FoundCrate::Name(name_string) => name_string,
        },
        Err(e) => panic!("Cannot find micro_test crate: {}", e),
    };
    let input = syn::parse_macro_input!(item as syn::Stmt);
    let process_macro = |old_macro: syn::Expr| {
        let mut expr_macro = match old_macro {
            syn::Expr::Macro(expr) => expr,
            _ => panic!("#[micro_test_assert] should only be applied to assert* macro invocation"),
        };
        transform_macro(&mut expr_macro.mac, micro_test_crate);
        syn::Expr::Macro(expr_macro)
    };
    let macro_expr = match input {
        syn::Stmt::Expr(expr) => syn::Stmt::Expr(process_macro(expr)),
        syn::Stmt::Semi(semi_expr, semi) => syn::Stmt::Semi(process_macro(semi_expr), semi),
        _ => panic!("#[micro_test_assert] should only be applied to assert* macro invocation"),
    };
    quote!(#macro_expr).into()
}
