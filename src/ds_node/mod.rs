pub mod ds_widget;
mod ds_if;
mod ds_iter;


use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Token;
pub use ds_widget::DsWidget;
use crate::ds_node::ds_if::DsIf;
use crate::ds_node::ds_iter::DsIter;

pub enum DsTree {
    Widget(Box<DsWidget>),
    If(Box<DsIf>),
    Iter(Box<DsIter>),
}

impl Parse for DsTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![if]) {
            Ok(DsTree::If(Box::new(input.parse()?)))
        } else if lookahead.peek(Token![for]) {
            Ok(DsTree::Iter(Box::new(input.parse()?)))
        } else {
            Ok(DsTree::Widget(Box::new(input.parse()?)))
        }
    }
}

impl ToTokens for DsTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DsTree::Widget(widget) => widget.to_tokens(tokens),
            DsTree::If(if_node) => if_node.to_tokens(tokens),
            DsTree::Iter(iter) => iter.to_tokens(tokens),
        }
    }
}
