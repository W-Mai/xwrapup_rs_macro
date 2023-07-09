use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Token;

use super::ds_traits::DsNodeIsMe;

pub struct DsIf;

impl Parse for DsIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DsIf)
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
