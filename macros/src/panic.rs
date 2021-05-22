use proc_macro2::TokenStream;
use quote::quote;

pub fn micro_panic_receiver_impl(item_fn: syn::ItemFn) -> TokenStream {
    // Get the name of micro_test crate
    let micro_test_crate_string = match proc_macro_crate::crate_name("micro_test") {
        Ok(founded_crate) => match founded_crate {
            proc_macro_crate::FoundCrate::Itself => String::from("micro_test"),
            proc_macro_crate::FoundCrate::Name(name_string) => name_string,
        },
        Err(e) => panic!("Cannot find micro_test crate: {}", e),
    };
    let micro_test_crate: syn::PathSegment = syn::parse_str(&micro_test_crate_string).unwrap();

    let attrs = item_fn.attrs.clone();
    let fn_vis = item_fn.vis.clone();
    let signature = item_fn.sig.clone();
    let block = item_fn.block.clone();

    let backtrace_type: syn::Type = syn::parse_quote! {
        #micro_test_crate::backtrace::CallStack
    };
    let backtrace_ident: syn::Ident = syn::parse_quote! {
        __micro_test_backtrace_call_stack
    };

    let new_fn = syn::ItemFn {
        attrs,
        vis: fn_vis,
        sig: signature,
        block: std::boxed::Box::new(syn::parse_quote! { {
            let #backtrace_ident = #backtrace_type::new();
            #block
        } })
    };
    quote! {
        #new_fn
    }
}

pub fn micro_panic_relay_impl(item_fn: syn::ItemFn) -> TokenStream {
    // Get the name of micro_test crate
    let micro_test_crate_string = match proc_macro_crate::crate_name("micro_test") {
        Ok(founded_crate) => match founded_crate {
            proc_macro_crate::FoundCrate::Itself => String::from("micro_test"),
            proc_macro_crate::FoundCrate::Name(name_string) => name_string,
        },
        Err(e) => panic!("Cannot find micro_test crate: {}", e),
    };
    let micro_test_crate: syn::PathSegment = syn::parse_str(&micro_test_crate_string).unwrap();

    let attrs = item_fn.attrs.clone();
    let fn_vis = item_fn.vis.clone();

    // Processing signature
    let mut signature = item_fn.sig.clone();
    let return_type = signature.output;
    // function name
    let backtrace_type: syn::Type = syn::parse_quote! {
        #micro_test_crate::backtrace::CallStack
    };
    // return type
    signature.output = match return_type {
        syn::ReturnType::Default => {
            syn::ReturnType::Type(
                syn::parse_quote!(->),
                Box::new(syn::parse_quote!(::core::result::Result<(), #backtrace_type>))
            )
        },
        syn::ReturnType::Type(r_arrow, ty) => {
            syn::ReturnType::Type(
                r_arrow,
                Box::new(syn::parse_quote!(::core::result::Result<#ty, #backtrace_type>))
            )
        }
    };
    let mut block = item_fn.block.clone();
    transform_block(&mut block);
    let new_fn = syn::ItemFn {
        attrs,
        vis: fn_vis,
        sig: signature,
        block,
    };
    quote! {
        #[cfg(not(test))]
        #item_fn
        #[cfg(test)]
        #new_fn
    }
}

fn transform_block(block: &mut syn::Block) {
    fn transform_expr(expr: &mut syn::Expr) {
        use syn::Expr::*;
        match expr {
            If(if_expr) => {
                transform_block(&mut if_expr.then_branch);
                match if_expr.else_branch.clone() {
                    Some((_, mut else_branch_expr)) => transform_expr(&mut else_branch_expr),
                    None => (),
                }
            },
            ForLoop(for_loop_expr) => transform_block(&mut for_loop_expr.body),
            Loop(loop_expr) => transform_block(&mut loop_expr.body),
            While(while_expr) => transform_block(&mut while_expr.body),
            Block(block_expr) => transform_block(&mut block_expr.block),
            Match(match_expr) => {
                let arms = &mut match_expr.arms;
                for arm in arms {
                    transform_expr(&mut *arm.body);
                }
            },
            Return(return_expr) => {
                let mut r_expr = return_expr.expr.clone();
                return_expr.expr = match &mut r_expr {
                    Some(boxed_expr) => Some(std::boxed::Box::new(syn::parse_quote! {
                        ::core::result::Result::Ok(#boxed_expr)
                    })),
                    None => Some(std::boxed::Box::new(syn::parse_quote! {
                        ::core::result::Result::Ok(())
                    }))
                }
            },
            _ => (),
        }
    }

    fn transform_stmt(stmt: &mut syn::Stmt) {
        match stmt {
            syn::Stmt::Expr(expr) => transform_expr(expr),
            syn::Stmt::Semi(semi_expr, _) => transform_expr(semi_expr),
            _ => (),
        }
    }

    let statements = &mut block.stmts;
    if let Some((last_stmt, others)) = statements.split_last_mut() {
        for statement in others {
            transform_stmt(statement);
        }
        match last_stmt {
            syn::Stmt::Expr(last_expr) => {
                *last_expr = syn::parse_quote! {
                    ::core::result::Result::Ok(#last_expr)
                }
            },
            _ => {
                statements.push(syn::parse_quote! {
                    #[allow(unreachable_code)]
                    {
                        return ::core::result::Result::Ok(());
                    }
                });
            }
        }
    } else {
        statements.push(syn::parse_quote! {
            #[allow(unreachable_code)]
            {
                return ::core::result::Result::Ok(());
            }
        });
    }
}
