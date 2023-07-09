use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Token;
use super::ds_traits::DsNodeIsMe;

pub struct DsIter;

impl Parse for DsIter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DsIter)
    }
}

impl ToTokens for DsIter {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {
            for _ in 0..10 {
                println!("Iter!");
            }
        });
    }
}

impl DsNodeIsMe for DsIter {
    fn is_me(input: ParseStream) -> bool {
        let lookahead = input.lookahead1();
        lookahead.peek(Token![for])
    }
}
