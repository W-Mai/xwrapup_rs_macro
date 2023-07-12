pub mod ds_widget;
mod ds_if;
mod ds_iter;
mod ds_traits;
mod ds_root;
mod ds_attr;

use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

pub use ds_root::DsRoot;
use ds_widget::DsWidget;
use ds_if::DsIf;
use ds_iter::DsIter;
use ds_traits::DsNodeIsMe;
use crate::ds_node::ds_traits::DsTreeToTokens;

#[derive(Debug)]
pub enum DsNodeType {
    Widget,
    If,
    Iter,
}

pub enum DsNode {
    Root(syn::Expr),
    Widget(DsWidget),
    If(DsIf),
    Iter(DsIter),
}

impl Debug for DsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // DsNode::Root(expr) => write!(f, "Root({:?})", expr.to_token_stream().to_string()),
            DsNode::Widget(widget) => write!(f, "Widget({:?})", widget),
            DsNode::If(if_node) => write!(f, "If({:?})", if_node),
            DsNode::Iter(iter) => write!(f, "Iter({:?})", iter),
            DsNode::Root(expr) => write!(f, "Root({:?})", expr.to_token_stream().to_string()),
        }
    }
}

#[derive(Debug)]
pub struct DsTree {
    parent: Option<Rc<RefCell<DsTree>>>,

    node: DsNode,

    children: Vec<Rc<RefCell<DsTree>>>,
}

impl DsTree {
    pub fn set_parent(&mut self, parent: Rc<RefCell<DsTree>>) {
        self.parent = Some(parent);
    }

    pub fn get_node(&self) -> &DsNode {
        &self.node
    }
}


impl DsNodeType {
    fn what_type(input: ParseStream) -> DsNodeType {
        if DsWidget::is_me(input) {
            DsNodeType::Widget
        } else if DsIf::is_me(input) {
            DsNodeType::If
        } else if DsIter::is_me(input) {
            DsNodeType::Iter
        } else {
            panic!("Unknown type of DsTree")
        }
    }
}

impl Parse for DsTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let node = DsNode::parse(input)?;

        let content;
        syn::braced!(content in input);

        let mut children = Vec::new();
        while !content.is_empty() {
            let child = Rc::new(RefCell::new(DsTree::parse(&content)?));
            child.borrow_mut().set_parent(Rc::clone(&child));
            children.push(child);
        }

        Ok(DsTree {
            parent: None,
            node,
            children,
        })
    }
}

impl DsTreeToTokens for DsTree {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, tree: &DsTree) {
        let DsTree { parent, node, children } = self;

        match parent {
            Some(parent) => {
                println!("parent: {:?}", parent.borrow().node);
            }
            None => {
                println!("parent: None");
            }
        }

        node.to_tokens(tokens, tree);

        for child in children.iter() {
            child.borrow().to_tokens(tokens, self);
        }
    }
}

impl Parse for DsNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tree_type = DsNodeType::what_type(input);

        let node = match tree_type {
            DsNodeType::Widget => DsNode::Widget(input.parse()?),
            DsNodeType::If => DsNode::If(input.parse()?),
            DsNodeType::Iter => DsNode::Iter(input.parse()?),
        };

        Ok(node)
    }
}

impl DsTreeToTokens for DsNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, tree: &DsTree) {
        match self {
            DsNode::Widget(widget) => widget.to_tokens(tokens, tree),
            DsNode::If(if_node) => if_node.to_tokens(tokens, tree),
            DsNode::Iter(iter) => iter.to_tokens(tokens, tree),
            _ => {}
        }
    }
}
