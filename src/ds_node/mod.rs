pub mod ds_widget;
mod ds_if;
mod ds_iter;
mod ds_traits;

use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub use ds_widget::DsWidget;
use ds_if::DsIf;
use ds_iter::DsIter;
use ds_traits::DsNodeIsMe;

pub enum DsTreeType {
    Widget,
    If,
    Iter,
}

pub enum DsTree {
    Widget(Box<DsWidget>),
    If(Box<DsIf>),
    Iter(Box<DsIter>),
}

impl DsTreeType {
    fn what_type(input: ParseStream) -> DsTreeType {
        if DsWidget::is_me(input) {
            DsTreeType::Widget
        } else if DsIf::is_me(input) {
            DsTreeType::If
        } else if DsIter::is_me(input) {
            DsTreeType::Iter
        } else {
            panic!("Unknown type of DsTree")
        }
    }
}

impl Parse for DsTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tree_type = DsTreeType::what_type(input);

        match tree_type {
            DsTreeType::Widget => Ok(DsTree::Widget(Box::new(input.parse()?))),
            DsTreeType::If => Ok(DsTree::If(Box::new(input.parse()?))),
            DsTreeType::Iter => Ok(DsTree::Iter(Box::new(input.parse()?))),
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
