extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{Expr, ExprLit, parse_macro_input};
use quote::{quote, ToTokens};
use syn::parse::Parse;

//! A widget.
//! ```
//! use xwrapup_rs_macro::ui;
//!
//! ui!(
//!     Widget (attr0: value0, attr1: value1, attr2: value2, ...) {
//!         ChildWidget0 (attr0: value0, attr1: value1, attr2: value2, ...) {
//!            ...
//!         }
//!         ChildWidget1 (attr0: value0, attr1: value1, attr2: value2, ...) {
//!           ...
//!         }
//!     }
//! )
//! ```

struct Attr {
    name: String,
    value: Expr,
}

struct Widget {
    name: String,

    attrs: Vec<Attr>,
    children: Vec<Widget>,
}

impl Parse for Widget {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();

        let mut attrs = Vec::new();
        let mut children = Vec::new();

        let params;
        if input.peek(syn::token::Paren) {
            syn::parenthesized!(params in input);
            while !params.is_empty() {
                let name = params.parse::<syn::Ident>()?.to_string();
                params.parse::<syn::Token![:]>()?;
                let value = params.parse::<syn::Expr>()?;

                if params.peek(syn::Token![,]) {
                    params.parse::<syn::Token![,]>()?;
                }
                attrs.push(Attr { name, value });
            }
        }

        let content;
        syn::braced!(content in input);

        while !content.is_empty() {
            if content.peek(syn::Ident) {
                children.push(content.parse()?);
            } else if content.peek(syn::Token![if]) {
                content.parse::<syn::Token![if]>()?;
                let cond = content.parse::<syn::Expr>()?;

                let if_content;
                syn::braced!(if_content in content);

                while !if_content.is_empty() {
                    children.push(if_content.parse()?);
                }
            }
        }
        Ok(Widget { name, attrs, children })
    }
}

impl ToTokens for Widget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Widget { name, attrs, children } = self;

        let token_string = quote! {
            {
                name: #name,
                attrs: [
                    #(#attrs),*
                ],
                children: [
                    #(#children),*
                ],
            }
        }.to_string();

        tokens.extend(quote! {
            #token_string
        });
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Attr { name, value } = self;

        let token_string = quote! {
            {
                name: #name,
                value: #value,
            }
        };

        tokens.extend(quote! {
            #token_string
        });
    }
}


#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Widget);

    TokenStream::from(input.to_token_stream())
}
