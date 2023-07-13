use quote::{quote};
use syn::parse::{Parse, ParseStream};
use syn::Token;
use crate::ds_node::ds_traits::DsTreeToTokens;
use crate::ds_node::DsTree;
use super::ds_traits::DsNodeIsMe;

#[derive(Debug)]
pub struct DsIter;

impl Parse for DsIter {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(DsIter)
    }
}

impl DsTreeToTokens for DsIter {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, _parent: &DsTree) {
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
