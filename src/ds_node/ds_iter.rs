use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use crate::ds_node::ds_context::{DsContext, DsContextRef};
use crate::ds_node::ds_custom_token;
use crate::ds_node::ds_traits::DsTreeToTokens;
use super::ds_traits::DsNodeIsMe;

pub struct DsIter {
    iterable: syn::Expr,
    variable: syn::Ident,
}

impl Debug for DsIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{iterable: {:?}, variable: {:?}}}",
               self.iterable.to_token_stream().to_string(),
               self.variable)
    }
}

impl Parse for DsIter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<ds_custom_token::walk>()?;
        let iterable = input.parse::<syn::Expr>()?;
        input.parse::<ds_custom_token::with>()?;
        let variable = input.parse::<syn::Ident>()?;

        Ok(DsIter {
            iterable,
            variable,
        })
    }
}

impl DsTreeToTokens for DsIter {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, ctx: DsContextRef) {
        let iterable = self.iterable.to_token_stream().to_string();
        let variable = &self.variable.to_token_stream().to_string();

        tokens.extend(quote! {
            println!("for {} in {} {{", #variable, #iterable);
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

impl DsNodeIsMe for DsIter {
    fn is_me(input: ParseStream) -> bool {
        input.peek(ds_custom_token::walk)
    }
}
