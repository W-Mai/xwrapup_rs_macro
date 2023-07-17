pub mod ds_widget;
mod ds_if;
mod ds_iter;
mod ds_traits;
mod ds_root;
mod ds_attr;
mod ds_context;
mod ds_node;
mod ds_custom_token;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};

pub use ds_root::DsRoot;
use ds_context::{DsContext, DsContextRef};
use ds_node::DsNode;
use ds_traits::DsTreeToTokens;
use macro_utils::DsRef;

#[derive(Debug, DsRef)]
pub struct DsTree {
    parent: Option<DsTreeRef>,

    node: DsNode,

    children: Vec<DsTreeRef>,
}

impl DsTree {
    pub fn set_parent(&mut self, parent: DsTreeRef) {
        self.parent = Some(parent);
    }

    pub fn get_node(&self) -> &DsNode {
        &self.node
    }
}

impl Parse for DsTree {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let node = DsNode::parse(input)?;

        let content;
        syn::braced!(content in input);

        let mut children = Vec::new();
        while !content.is_empty() {
            let child = DsTree::parse(&content)?.into_ref();
            child.borrow_mut().set_parent(child.clone());
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, ctx: DsContextRef) {
        let DsTree { parent, node, children: _children } = self;

        let ctx = DsContext {
            parent: parent.clone(),
            tree: ctx.borrow().tree.clone(),
        }.into_ref();

        node.to_tokens(tokens, ctx);
    }
}
