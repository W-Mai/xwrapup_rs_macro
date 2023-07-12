use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use crate::ds_node::ds_traits::ToTokensWithContext;
use crate::ds_node::ds_widget::DsWidget;

use crate::ds_node::DsTree;
use super::ds_attr::DsAttrs;

#[derive(Clone)]
pub struct DsRoot {
    // only support parent now
    parent: syn::Expr,

    content: DsTree,
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
                    content: DsTree::parse(input)?,
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

        let parent_string = self.parent.to_token_stream().to_string();

        tokens.extend(quote! {
            println!("let {} = {:?}", #parent_string, #parent);
        });

        content.to_tokens_with_context(tokens, DsTree::Root(self));
    }
}
