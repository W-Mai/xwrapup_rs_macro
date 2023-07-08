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
//!         if (cond) {
//!            ...
//!         }
//!     }
//! )
//! ```

extern crate proc_macro;


use proc_macro::TokenStream;
use syn::{Expr, parse_macro_input, Ident};
use quote::{quote, ToTokens};
use syn::parse::Parse;


struct Attr {
    name: Ident,
    value: Expr,
}

struct Widget {
    name: Ident,

    attrs: Vec<Attr>,
    children: Vec<Widget>,
}

impl Parse for Widget {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;

        let mut attrs = Vec::new();
        let mut children = Vec::new();

        let params;
        if input.peek(syn::token::Paren) {
            syn::parenthesized!(params in input);
            while !params.is_empty() {
                let name = params.parse::<syn::Ident>()?;
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

        let name_string = name.to_string();

        let token_string = quote! {
            println!("{{");
            println!("  name: {}", #name_string);
            println!("  attrs: [");
            #( #attrs )*
            println!("  ],");
            println!("  children: [");
            #( #children )*
            println!("  ],");
            println!("}}");
        };

        tokens.extend(quote! {
            #token_string
        });
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Attr { name, value } = self;
        let name_string = name.to_string();
        let token_string = quote! {
            println!("{{ name: {}, value: {} }},", #name_string, #value);
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
