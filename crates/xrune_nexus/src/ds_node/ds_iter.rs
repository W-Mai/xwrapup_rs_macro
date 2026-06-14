use super::ds_traits::DsNodeIsMe;
use crate::ds_node::ds_custom_token;
use quote::ToTokens;
use std::fmt::Debug;
use syn::parse::{Parse, ParseStream};

pub struct DsIter {
    iterable: syn::Expr,
    variable: syn::Ident,
    reactive: bool,
    key: Option<Box<syn::Expr>>,
}

impl DsIter {
    pub fn get_iterable(&self) -> &syn::Expr {
        &self.iterable
    }

    pub fn get_variable(&self) -> &syn::Ident {
        &self.variable
    }

    pub fn is_reactive(&self) -> bool {
        self.reactive
    }

    pub fn get_key(&self) -> Option<&syn::Expr> {
        self.key.as_deref()
    }
}

impl Debug for DsIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{iterable: {:?}, variable: {:?}}}",
            self.iterable.to_token_stream().to_string(),
            self.variable
        )
    }
}

impl Parse for DsIter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<ds_custom_token::walk>()?;
        let (iterable, reactive) = super::reactive::reactive_attr_or_expr(input)?;
        input.parse::<ds_custom_token::with>()?;
        let variable = input.parse::<syn::Ident>()?;
        let key = if input.peek(ds_custom_token::by) {
            input.parse::<ds_custom_token::by>()?;
            Some(Box::new(super::reactive::collect_until_brace(input)?))
        } else {
            None
        };
        Ok(DsIter {
            iterable,
            variable,
            reactive,
            key,
        })
    }
}

impl DsNodeIsMe for DsIter {
    fn is_me(input: ParseStream) -> bool {
        input.peek(ds_custom_token::walk)
    }
}
