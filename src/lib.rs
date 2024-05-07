use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, Expr, ExprArray, ExprIndex, ItemFn};

#[proc_macro]
pub fn unchecked_indices_ref(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as ExprArray);

    let transformed_elements = expr.elems.into_iter().map(|element| {
        if let Expr::Index(ExprIndex { expr, index, .. }) = element {
            quote! {
                unsafe { #expr.get_unchecked(#index) }
            }
        } else {
            quote! {
                #element
            }
        }
    });

    let output = quote! {
        [#(#transformed_elements),*]
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn unchecked_indices_clone(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as ExprArray);

    let transformed_elements = expr.elems.into_iter().map(|element| {
        if let Expr::Index(ExprIndex { expr, index, .. }) = element {
            quote! {
                unsafe { #expr.get_unchecked(#index).clone() }
            }
        } else {
            quote! {
                #element
            }
        }
    });

    let output = quote! {
        [#(#transformed_elements),*]
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn unchecked_indices_copy(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as ExprArray);

    let transformed_elements = expr.elems.into_iter().map(|element| {
        if let Expr::Index(ExprIndex { expr, index, .. }) = element {
            quote! {
                unsafe { *#expr.get_unchecked(#index) }
            }
        } else {
            quote! {
                #element
            }
        }
    });

    let output = quote! {
            [#(#transformed_elements),*]
    };

    TokenStream::from(output)
}

struct IndexAccessVisitor;

impl VisitMut for IndexAccessVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // Use a recursive approach to handle nested expressions
        if let Expr::Index(index_expr) = expr {
            let expr_clone = *index_expr.expr.clone();
            let index_clone = *index_expr.index.clone();

            // Replace the indexed expression with a .get_unchecked() call inside an unsafe block
            *expr = syn::parse_quote! {
                unsafe { *#expr_clone.get_unchecked(#index_clone) }
            };
        } else {
            // Continue visiting recursively
            syn::visit_mut::visit_expr_mut(self, expr);
        }
    }
}

#[proc_macro_attribute]
pub fn unsafe_access_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    // Ensure the function is marked unsafe
    if func.sig.unsafety.is_none() {
        return syn::Error::new_spanned(&func.sig.fn_token, "Function must be declared unsafe")
            .to_compile_error()
            .into();
    }

    let mut visitor = IndexAccessVisitor;
    visitor.visit_item_fn_mut(&mut func);

    let output = quote! {
        #func
    };

    TokenStream::from(output)
}
