use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use crate::ds_node::ds_context::{DsContext, DsContextRef};
use crate::ds_node::ds_custom_token::is_custom_keyword;
use super::ds_traits::DsTreeToTokens;
use super::ds_node::DsNode;
use super::ds_attr::DsAttrs;
use super::ds_traits::DsNodeIsMe;

#[derive(Debug)]
pub struct DsWidget {
    name: syn::Ident,

    attrs: DsAttrs,
}

impl DsWidget {
    pub fn get_name(&self) -> &syn::Ident {
        &self.name
    }

    pub fn get_attrs(&self) -> &DsAttrs {
        &self.attrs
    }
}

impl Parse for DsWidget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;

        let attrs = input.parse::<DsAttrs>()?;

        Ok(DsWidget { name, attrs })
    }
}

impl DsTreeToTokens for DsWidget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, ctx: DsContextRef) {
        let DsWidget { name, attrs } = self;

        let parent = ctx.borrow().parent.clone().unwrap();

        let parent_string = match parent.borrow().get_node() {
            DsNode::Widget(widget) => {
                let widget_name = &widget.name;
                quote! { #widget_name }
            }
            DsNode::Root(root) => {
                let root_name = &root.to_token_stream();
                quote! { #root_name }
            }
            _ => {
                quote! { "?" }
            }
        }.to_string();
        let name_string = name.to_string();

        tokens.extend(quote! {
            println!("let {} = obj::new({})", #name_string, #parent_string);
        });

        for attr in attrs.attrs.iter() {
            let attr_name = &attr.name.to_string();
            let attr_value = &attr.value;
            tokens.extend(quote! {
                println!("{}.set_{}({:?})", #name_string, #attr_name, #attr_value);
            });
        }

        let tree = ctx.borrow().tree.clone();
        let children = &tree.borrow().children;

        for child in children.iter() {
            let ctx = DsContext {
                parent: Some(ctx.borrow().tree.clone()),
                tree: child.clone(),
            }.into_ref();
            child.borrow().to_tokens(tokens, ctx);
        }
    }
}

impl DsNodeIsMe for DsWidget {
    fn is_me(input: ParseStream) -> bool {
        input.peek(syn::Ident) && !is_custom_keyword(input)
    }
}
