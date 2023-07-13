use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::ds_node::DsTree;
use crate::ds_node::ds_context::{DsContextRef};
use crate::ds_node::ds_node::DsNode;
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
                content.set_parent(
                    DsTree {
                        parent: None,
                        node: DsNode::Root(parent.clone()),
                        children: vec![],
                    }.into_ref()
                );
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

        let fake_tree = DsTree {
            parent: None,
            node: DsNode::Root(parent.clone()),
            children: vec![],
        }.into_ref();

        let ctx = DsContextRef::new(Some(fake_tree.clone()), fake_tree.clone());

        content.to_tokens(tokens, ctx);
    }
}
