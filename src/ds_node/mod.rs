pub mod ds_widget;
mod ds_if;
mod ds_iter;
mod ds_traits;
mod ds_root;
mod ds_attr;
mod ds_context;
mod ds_node;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use syn::parse::{Parse, ParseStream};

pub use ds_root::DsRoot;
use ds_context::{DsContext, DsContextRef};
use ds_node::DsNode;
use ds_traits::DsTreeToTokens;

#[derive(Debug)]
pub struct DsTree {
    parent: Option<DsTreeRef>,

    node: DsNode,

    children: Vec<DsTreeRef>,
}

#[derive(Debug)]
pub struct DsTreeRef {
    inner: Rc<RefCell<DsTree>>,
}

impl DsTree {
    pub fn set_parent(&mut self, parent: DsTreeRef) {
        self.parent = Some(parent);
    }

    pub fn get_node(&self) -> &DsNode {
        &self.node
    }

    pub fn into_ref(self) -> DsTreeRef {
        DsTreeRef {
            inner: Rc::new(RefCell::new(self)),
        }
    }
}

impl DsTreeRef {
    pub fn borrow(&self) -> std::cell::Ref<DsTree> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<DsTree> {
        self.inner.borrow_mut()
    }
}

impl Clone for DsTreeRef {
    fn clone(&self) -> Self {
        DsTreeRef {
            inner: Rc::clone(&self.inner),
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
        let DsTree { parent: _parent, node, children } = self;

        node.to_tokens(tokens, ctx.clone());

        for child in children.iter() {
            let ctx = DsContext {
                parent: Some(ctx.borrow().tree.clone()),
                tree: child.clone(),
            }.into_ref();
            child.borrow().to_tokens(tokens, ctx);
            tokens.extend(quote::quote! { println!(); });
        }
    }
}
