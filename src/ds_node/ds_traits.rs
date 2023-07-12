use syn::parse::ParseStream;
use crate::ds_node::DsTree;

pub trait DsNodeIsMe {
    fn is_me(input: ParseStream) -> bool;
}

pub trait DsTreeToTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, parent: &DsTree);
}
