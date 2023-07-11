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

pub struct DsContext {
    pub parent: Option<Box<&'static DsTree>>,
}

pub struct DsNode {
    pub ctx: DsContext,

    pub tree: Box<DsTree>,
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {}
}

impl DsNode {
    pub fn new(tree: DsTree) -> DsNode {
        DsNode {
            ctx: DsContext { parent: None },
            tree: Box::new(tree),
        }
    }

    pub fn get_context(&self) -> &DsContext {
        &self.ctx
    }

    pub fn get_tree(&self) -> &DsTree {
        &self.tree
    }

    pub fn set_context(&mut self, ctx: DsContext) {
        self.ctx = ctx;
    }
}

impl Parse for DsNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DsNode::new(input.parse()?))
    }
}

impl ToTokens for DsNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.tree.as_ref() {
            DsTree::Widget(widget) => {
                let widget = widget;
                widget.to_tokens(tokens)
            }
            DsTree::If(if_node) => if_node.to_tokens(tokens),
            DsTree::Iter(iter) => iter.to_tokens(tokens),
        }
    }
}
