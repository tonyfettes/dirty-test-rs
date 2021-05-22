use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromMeta)]
struct PathTargetFeatureAttrArgs {
    pub path: bool,
    pub target: String,
    pub feature: String,
}

#[derive(FromMeta)]
struct TargetFeatureAttrArgs {
    pub target: String,
    pub feature: String,
}

#[derive(FromMeta)]
struct PathTargetAttrArgs {
    pub path: bool,
    pub target: String,
}

#[derive(FromMeta)]
struct TargetAttrArgs {
    pub target: String,
}

#[derive(FromMeta)]
struct PathAttrArgs {
    pub path: bool,
}

#[derive(FromMeta)]
struct EmptyAttrArgs;

#[allow(unused_macros)]
macro_rules! attr_fallback {
    (@forward $input:ident, EmptyAttrArgs) => {
        { ProcMacroAttrArgs { path: false, target: None, feature: None, } }
    };
    (@forward $input:ident, PathAttrArgs, $next:expr) => {
        { match PathAttrArgs::from_list(&$input) { Ok(v) => ProcMacroAttrArgs { path: v.path, target: None, feature: None, }, Err(_) => { $next } } }
    };
    (@forward $input:ident, TargetAttrArgs, $next:expr) => {
        { match TargetAttrArgs::from_list(&$input) { Ok(v) => ProcMacroAttrArgs { path: false, target: Some(v.target), feature: None, }, Err(_) => { $next } } }
    };
    (@forward $input:ident, PathTargetAttrArgs, $next:expr) => {
        { match PathTargetAttrArgs::from_list(&$input) { Ok(v) => ProcMacroAttrArgs { path: v.path, target: Some(v.target), feature: None, }, Err(_) => { $next } } }
    };
    (@forward $input:ident, TargetFeatureAttrArgs, $next:expr) => {
        { match TargetFeatureAttrArgs::from_list(&$input) { Ok(v) => ProcMacroAttrArgs { path: false, target: Some(v.target), feature: Some(v.feature), }, Err(_) => { $next } } }
    };
    (@forward $input:ident, PathTargetFeatureAttrArgs, $next:expr) => {
        { match PathTargetFeatureAttrArgs::from_list(&$input) { Ok(v) => ProcMacroAttrArgs { path: v.path, target: Some(v.target), feature: Some(v.feature), }, Err(_) => { $next } } }
    };
    ($input:expr) => {
        {
            let attr_input = $input;
            attr_fallback!(@forward attr_input, PathTargetFeatureAttrArgs,
                attr_fallback!(@forward attr_input, TargetFeatureAttrArgs,
                    attr_fallback!(@forward attr_input, PathTargetAttrArgs,
                        attr_fallback!(@forward attr_input, TargetAttrArgs,
                            attr_fallback!(@forward attr_input, PathAttrArgs,
                                attr_fallback!(@forward attr_input, EmptyAttrArgs))))))
        }
    }
}

pub struct ProcMacroAttrArgs {
    pub path: bool,
    pub target: Option<String>,
    pub feature: Option<String>,
}

const METADATA_PROCESSOR_NAME: &'static str = "report_metadata";

pub fn micro_test_case_impl(attr_args: Vec<syn::NestedMeta>, item: TokenStream) -> TokenStream {
    let attr_args = attr_fallback!(attr_args);
    // Get the name of micro_test crate
    let micro_test_crate_string = match proc_macro_crate::crate_name("micro_test") {
        Ok(founded_crate) => match founded_crate {
            proc_macro_crate::FoundCrate::Itself => String::from("micro_test"),
            proc_macro_crate::FoundCrate::Name(name_string) => name_string,
        },
        Err(e) => panic!("Cannot find micro_test crate: {}", e),
    };

    let mut input = syn::parse2::<syn::ItemFn>(item).unwrap();

    let mut is_ignored = true;
    let attrs = input.attrs.clone();
    for attr in attrs {
        if attr.path.segments.len() == 1 {
            if attr.path.segments.last().unwrap().ident.to_string() == "micro_ignore" {
                is_ignored = true;
            }
        } else if attr.path.segments.len() == 2 {
            if attr.path.segments.last().unwrap().ident.to_string() == "micro_ignore" &&
                attr.path.segments.first().unwrap().ident.to_string() == micro_test_crate_string {
                is_ignored = true;
            }
        }
    }

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

    // Set default values for attributes.
    let target = match attr_args.target {
        Some(target) => if attr_args.path { "::".to_owned() + &target } else { target },
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
    let metadata_processor = syn::Ident::from_string(METADATA_PROCESSOR_NAME).unwrap();

    input.block = if attr_args.path {
        syn::parse_quote! {
            {
                #micro_test_crate::test::#metadata_processor(&#micro_test_crate::test::Metadata {
                    target: ::core::concat!(::core::module_path!(), #target),
                    feature: #feature,
                });
                #block
            }
        }
    } else {
        syn::parse_quote! {
            {
                #micro_test_crate::test::#metadata_processor(&#micro_test_crate::test::Metadata {
                    target: #target,
                    feature: #feature,
                });
                #block
            }
        }
    };
    //input.block = Box::new(syn::parse2::<syn::Block>(new_block).unwrap());
    if is_ignored {
        quote! {
            #[#micro_test_crate::panic::micro_panic_relay]
            #[test_case]
            #input
        }
    } else {
        quote! {
            #[#micro_test_crate::panic::micro_panic_relay]
            #input
        }
    }
}

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
    }
}

#[cfg(not(feature = "replace_assert"))]
fn transform_macro(mac: &mut syn::Macro, micro_test_crate: String) {
    let old_string = mac.path.segments.last().unwrap().ident.to_string();
    if old_string.len() == 2 {
        assert_eq!(mac.path.segments.first().unwrap().ident.to_string(), micro_test_crate);
    }
}

fn transform_block(block: &mut syn::Block, micro_test_crate: &String) {
    fn transform_expr(expr: &mut syn::Expr, micro_test_crate: &String) {
        use syn::Expr::*;
        match expr {
            Macro(macro_expr) => transform_macro(&mut macro_expr.mac, micro_test_crate.clone()),
            If(if_expr) => {
                transform_block(&mut if_expr.then_branch, micro_test_crate);
                match if_expr.else_branch.clone() {
                    Some((_, mut else_branch_expr)) => transform_expr(&mut else_branch_expr, micro_test_crate),
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

    let statements = &mut block.stmts;
    for mut statement in statements {
        transform_stmt(&mut statement, micro_test_crate);
    }
}
