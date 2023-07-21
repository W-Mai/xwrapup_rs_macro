extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};
use xwrapup_rs_macro::ds_node::DsRoot;

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DsRoot);

    TokenStream::from(input.to_token_stream())
}

