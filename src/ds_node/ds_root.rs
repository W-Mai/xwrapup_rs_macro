use std::fmt::Debug;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

use crate::ds_node::{DsTree, DsTreeRef};
use crate::ds_node::ds_context::DsContextRef;
use crate::ds_node::ds_node::DsNode;
use crate::ds_node::ds_attr::DsAttr;
use crate::ds_node::ds_traits::DsTreeToTokens;

pub struct DsRoot {
    // only support parent now
    parent: syn::Expr,

    content: DsTreeRef,
}

impl Debug for DsRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DsRoot { parent, content } = self;

        f.debug_struct("DsRoot")
            .field("parent", &parent.span().unwrap())
            .field("content", content)
            .finish()
    }
}

impl Parse for DsRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;

            let mut attrs = Vec::<DsAttr>::new();
            let params;
            syn::parenthesized!(params in input);
            while !params.is_empty() {
                attrs.push(params.parse()?);
                if params.peek(syn::Token![:]) {
                    params.parse::<syn::Token![:]>()?;
                }
            }

            let mut iter = attrs.iter();
            if let Some(parent_index) = iter.position(|attr| attr.name == "parent") {
                let parent = attrs[parent_index].value.clone();

                let content = DsTree::parse(input)?.into_ref();
                content.borrow_mut().set_parent(
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

        let ctx = DsContextRef::new(content.borrow().parent.clone(), content.clone());

        content.borrow().to_tokens(tokens, ctx);
    }
}
