use std::cell::RefCell;
use std::rc::Rc;
use proc_macro2::TokenStream;
use quote::quote;
use crate::ui_code_gen::{UcgToTokens};

pub struct XwUcgWidget {
    tokens: Rc<RefCell<TokenStream>>,
}


impl UcgToTokens for XwUcgWidget {
    fn new(tokens: TokenStream) -> Self {
        XwUcgWidget {
            tokens: Rc::new(RefCell::new(tokens)),
        }
    }

    fn gen_widget(&self, widget_type: &syn::Ident, parent: String) {
        let widget_type = widget_type.to_string();
        let parent = parent.to_string();
        let new_tokens = quote! {
            let widget = #widget_type::new();
            #parent.add_child(widget);
        };
        self.tokens.borrow_mut().extend(new_tokens);
    }
}
