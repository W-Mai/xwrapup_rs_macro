pub mod ds_widget;
pub mod ds_context;
pub mod ds_if;
pub mod ds_iter;
pub mod ds_traits;
pub mod ds_root;
pub mod ds_attr;
pub mod ds_node;
pub mod ds_custom_token;

use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};

pub use ds_root::DsRoot;
use ds_context::{DsContext, DsContextRef};
use ds_node::DsNode;
use ds_traits::DsTreeToTokens;
use proc_macros_inner::DsRef;

#[derive(DsRef)]
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

impl Debug for DsTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parent = match &self.parent {
            None => { "None" }
            Some(tree) => {
                match tree.borrow().node {
                    DsNode::Root(_) => { "Root" }
                    DsNode::Widget(_) => { "Widget" }
                    DsNode::If(_) => { "If" }
                    DsNode::Iter(_) => { "Iter" }
                }
            }
        };
        f.write_fmt(format_args!("{{ parent: {}, node: {:?}, children: {:?} }}", parent, self.node, self.children))
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
