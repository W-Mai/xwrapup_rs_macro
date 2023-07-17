use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use crate::ds_node::ds_context::DsContext;
use super::ds_context::DsContextRef;
use super::ds_traits::DsTreeToTokens;

use super::ds_traits::DsNodeIsMe;

pub struct DsIf {
    condition: syn::Expr,
}

impl Debug for DsIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "If({:?})", self.condition.to_token_stream().to_string())
    }
}


impl Parse for DsIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![if]) {
            input.parse::<syn::Token![if]>()?;
            let condition = input.parse::<syn::Expr>()?;

            Ok(DsIf {
                condition,
            })
        } else {
            Err(syn::Error::new_spanned(
                quote!(if),
                "Expected `if` or `else if`",
            ))
        }
    }
}

impl DsTreeToTokens for DsIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, ctx: DsContextRef) {
        let con = self.condition.to_token_stream().to_string();

        tokens.extend(quote! {
            println!("if {} {{", #con);
        });

        let tree = ctx.borrow().tree.clone();
        let children = &tree.borrow().children;

        for child in children.iter() {
            println!("child: {:?}", child.borrow().node);
            let ctx = DsContext {
                parent: Some(ctx.borrow().tree.clone()),
                tree: child.clone(),
            }.into_ref();
            child.borrow().to_tokens(tokens, ctx);
        }

        tokens.extend(quote! {
            println!("}}");
        });
    }
}

impl DsNodeIsMe for DsIf {
    fn is_me(input: ParseStream) -> bool {
        input.peek(syn::Token![if])
    }
}
