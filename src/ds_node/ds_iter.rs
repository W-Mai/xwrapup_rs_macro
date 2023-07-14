use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use crate::ds_node::ds_context::DsContextRef;
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, _ctx: DsContextRef) {
        let iterable = self.iterable.to_token_stream().to_string();
        let variable = &self.variable.to_token_stream().to_string();

        tokens.extend(quote! {
            println!("for {} in {} {{", #variable, #iterable);
                println!("\tIter!");
            println!("}}");
        });
    }
}

impl DsNodeIsMe for DsIter {
    fn is_me(input: ParseStream) -> bool {
        input.peek(ds_custom_token::walk)
    }
}
