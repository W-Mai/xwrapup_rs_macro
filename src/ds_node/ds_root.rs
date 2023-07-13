use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::ds_node::{DsContext, DsNode, DsTree};
use crate::ds_node::ds_traits::DsTreeToTokens;
use super::ds_attr::DsAttrs;

pub struct DsRoot {
    // only support parent now
    parent: syn::Expr,

    content: DsTree,
}

impl Debug for DsRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DsRoot { parent, content } = self;

        f.debug_struct("DsRoot")
            .field("parent", &parent.into_token_stream().to_string())
            .field("content", content)
            .finish()
    }
}

impl Parse for DsRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;

            let attrs = input.parse::<DsAttrs>()?;
            let mut iter = attrs.attrs.iter();
            if let Some(parent_index) = iter.position(|attr| attr.name == "parent") {
                let parent = attrs.attrs[parent_index].value.clone();

                let mut content = DsTree::parse(input)?;
                content.set_parent(Rc::new(RefCell::new(
                    DsTree {
                        parent: None,
                        node: DsNode::Root(parent.clone()),
                        children: vec![],
                    }
                )));
                return Ok(DsRoot {
                    parent,
                    content,
                });
            }
        }

        Err(syn::Error::new(
            input.span(),
            "Root node must have a parent",
        ))
    }
}


impl ToTokens for DsRoot {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DsRoot { parent, content } = self;

        let parent_string = "parent".to_string();

        tokens.extend(quote! {
            println!("let {} = {:?}", #parent_string, #parent);
        });

        let ctx = DsContext {
            parent: Some(Rc::new(RefCell::new(
                DsTree {
                    parent: None,
                    node: DsNode::Root(parent.clone()),
                    children: vec![],
                }
            ))),
            tree: Rc::new(RefCell::new(DsTree {
                parent: None,
                node: DsNode::Root(parent.clone()),
                children: vec![],
            })),
        };

        let ctx = Rc::new(RefCell::new(ctx));

        content.to_tokens(tokens, ctx);
    }
}
