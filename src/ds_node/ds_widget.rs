use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use super::ds_traits::DsTreeToTokens;
use super::DsNode;
use super::ds_attr::DsAttrs;
use super::ds_traits::DsNodeIsMe;
use super::DsTree;

#[derive(Debug)]
pub struct DsWidget {
    name: syn::Ident,

    attrs: DsAttrs,
}

impl Parse for DsWidget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;


        let attrs = input.parse::<DsAttrs>()?;

        Ok(DsWidget { name, attrs })
    }
}

impl DsTreeToTokens for DsWidget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, parent: &DsTree) {
        let DsWidget { name, attrs } = self;


        let parent_string = match parent.get_node() {
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
    }
}

impl DsNodeIsMe for DsWidget {
    fn is_me(input: ParseStream) -> bool {
        let lookahead = input.lookahead1();
        lookahead.peek(syn::Ident)
    }
}
