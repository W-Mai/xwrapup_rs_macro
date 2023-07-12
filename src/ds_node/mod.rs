pub mod ds_widget;
mod ds_if;
mod ds_iter;
mod ds_traits;
mod ds_root;
mod ds_attr;

use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub use ds_root::DsRoot;
use ds_widget::DsWidget;
use ds_if::DsIf;
use ds_iter::DsIter;
use ds_traits::DsNodeIsMe;
use crate::ds_node::ds_traits::ToTokensWithContext;

pub enum DsTreeType {
    Widget,
    If,
    Iter,
}

#[derive(Clone)]
pub enum DsTree {
    Root(&'static DsRoot),
    Widget(&'static DsWidget),
    If(&'static DsIf),
    Iter(&'static DsIter),
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
            DsTreeType::Widget => Ok(DsTree::Widget(&input.parse()?)),
            DsTreeType::If => Ok(DsTree::If(&input.parse()?)),
            DsTreeType::Iter => Ok(DsTree::Iter(&input.parse()?)),
        }
    }
}

impl ToTokens for DsTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // self.to_tokens_with_context(tokens, DsTree::Widget(DsWidget::default()));
    }
}

impl ToTokensWithContext for DsTree {
    fn to_tokens_with_context(&self, tokens: &mut proc_macro2::TokenStream, parent: DsTree) {
        match self {
            DsTree::Widget(widget) => widget.to_tokens_with_context(tokens, parent),
            DsTree::If(if_node) => if_node.to_tokens_with_context(tokens, parent),
            DsTree::Iter(iter) => iter.to_tokens_with_context(tokens, parent),
            _ => {}
        }
    }
}
