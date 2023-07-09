use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

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
