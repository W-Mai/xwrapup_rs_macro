use std::fmt::Debug;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::Expr;
use syn::parse::{Parse, ParseStream};

pub struct DsAttr {
    pub name: Ident,
    pub value: Expr,
}

impl Debug for DsAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DsAttr { name, value } = self;
        write!(f, "DsAttr {{ name: {}, value: {:?} }}", name, value.to_token_stream().to_string())
    }
}

#[derive(Debug)]
pub struct DsAttrs {
    pub attrs: Vec<DsAttr>,
}

impl Parse for DsAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![:]>()?;
        let value = input.parse::<syn::Expr>()?;

        Ok(DsAttr { name, value })
    }
}

impl Parse for DsAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();

        let params;
        if input.peek(syn::token::Paren) {
            syn::parenthesized!(params in input);
            while !params.is_empty() {
                attrs.push(params.parse()?);
                if params.peek(syn::Token![,]) {
                    params.parse::<syn::Token![,]>()?;
                }
            }
        }

        Ok(DsAttrs { attrs })
    }
}

impl ToTokens for DsAttr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DsAttr { name, value } = self;
        let name_string = name.to_string();
        let token_string = quote! {
            println!("setAttribute({}, {})", #name_string, stringify!(#value));
        };

        tokens.extend(quote! {
            #token_string
        });
    }
}
