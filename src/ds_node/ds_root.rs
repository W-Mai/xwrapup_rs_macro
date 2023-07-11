use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::ds_node::{DsNode, DsTree};
use super::ds_attr::DsAttrs;

pub struct DsRoot {
    // only support parent now
    parent: syn::Expr,

    content: DsNode,
}

impl Parse for DsRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;

            let attrs = input.parse::<DsAttrs>()?;
            let mut iter = attrs.attrs.iter();
            if let Some(parent_index) = iter.position(|attr| attr.name == "parent") {
                let parent = attrs.attrs[parent_index].value.clone();

                return Ok(DsRoot {
                    parent,
                    content: DsNode::parse(input)?,
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

        tokens.extend(quote! {
            println!("let {} = {:?}", "parent", #parent);
        });

        content.to_tokens(tokens);
    }
}
