use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Token;
use crate::ds_node::ds_traits::ToTokensWithContext;
use crate::ds_node::DsTree;
use super::ds_traits::DsNodeIsMe;

#[derive(Clone)]
pub struct DsIter;

impl Parse for DsIter {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
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

impl ToTokensWithContext for DsIter {
    fn to_tokens_with_context(&self, tokens: &mut proc_macro2::TokenStream, context: DsTree) {
        tokens.extend(quote! {
            for _ in 0..10 {
                println!("Iter! {}", #context);
            }
        });
    }
}
