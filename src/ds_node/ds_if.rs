use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Token;
use crate::ds_node::DsTree;

use super::ds_traits::DsNodeIsMe;

#[derive(Debug)]
pub struct DsIf;

impl Parse for DsIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![if]) {
            input.parse::<syn::Token![if]>()?;
            let _ = input.parse::<syn::Expr>()?;

            Ok(DsIf)
        } else {
            Err(syn::Error::new_spanned(
                DsIf,
                "Expected `if` or `else if`",
            ))
        }
    }
}

impl ToTokens for DsIf {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {
            if true {
                println!("If!");
            }
        });
    }
}

impl DsNodeIsMe for DsIf {
    fn is_me(input: ParseStream) -> bool {
        let lookahead = input.lookahead1();
        lookahead.peek(Token![if])
    }
}
