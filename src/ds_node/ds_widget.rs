use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use super::ds_attr::DsAttrs;
use super::ds_traits::DsNodeIsMe;
use super::DsTree;

pub struct DsWidget {
    name: syn::Ident,

    attrs: DsAttrs,
    children: Vec<DsTree>,
}

impl Parse for DsWidget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;


        let attrs = input.parse::<DsAttrs>()?;

        let content;
        syn::braced!(content in input);

        let mut children = Vec::new();
        while !content.is_empty() {
            children.push(DsTree::parse(&content)?);
        }

        Ok(DsWidget { name, attrs, children })
    }
}

impl ToTokens for DsWidget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DsWidget { name, attrs, children } = self;

        let parent_string = "parent".to_string();
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

        for child in children {
            child.to_tokens(tokens);
        }
    }
}

impl DsNodeIsMe for DsWidget {
    fn is_me(input: ParseStream) -> bool {
        let lookahead = input.lookahead1();
        lookahead.peek(syn::Ident)
    }
}
