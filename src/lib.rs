//! A widget.
//! ```
//! use xwrapup_rs_macro::ui;
//!
//! ui! {
//!     :(parent: parent)
//!
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
//! }
//!
//! ```

extern crate proc_macro;

mod ds_node;
mod ui_code_gen;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use ds_node::DsRoot;


#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DsRoot);

    TokenStream::from(input.to_token_stream())
}
