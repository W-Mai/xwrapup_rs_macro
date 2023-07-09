use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

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
