use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::Expr;
use syn::parse::Parse;

pub struct Attr {
    name: Ident,
    value: Expr,
}

pub struct Widget {
    name: Ident,

    parent: Ident,

    attrs: Vec<Attr>,
    children: Vec<Widget>,
}

impl Parse for Widget {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Widget::parse_with_parent(input, Ident::new("screen", proc_macro2::Span::call_site()))
    }
}

impl Widget {
    fn parse_with_parent(input: syn::parse::ParseStream, parent: Ident) -> syn::Result<Self> {
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
                children.push(Widget::parse_with_parent(&content, name.clone())?);
            } else if content.peek(syn::Token![if]) {
                content.parse::<syn::Token![if]>()?;
                let _ = content.parse::<syn::Expr>()?;

                let if_content;
                syn::braced!(if_content in content);

                while !if_content.is_empty() {
                    children.push(if_content.parse()?);
                }
            }
        }

        Ok(Widget { name, parent, attrs, children })
    }
}

impl ToTokens for Widget {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Widget { name, parent, attrs, children } = self;

        let parent_string = parent.to_string();
        let name_string = name.to_string();

        tokens.extend(quote! {
            println!("let {} = obj::new({})", #name_string, #parent_string);
        });

        for attr in attrs {
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

impl ToTokens for Attr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Attr { name, value } = self;
        let name_string = name.to_string();
        let token_string = quote! {
            println!("setAttribute({}, {})", #name_string, stringify!(#value));
        };

        tokens.extend(quote! {
            #token_string
        });
    }
}
