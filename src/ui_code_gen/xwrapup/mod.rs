use proc_macro2::TokenStream;
use quote::quote;
use crate::ds_node::ds_context::DsContextRef;
use crate::ds_node::DsTreeRef;
use crate::ui_code_gen::ucg_widget::UcgWidget;
use crate::ui_code_gen::{UcgToTokens, UcgType};

pub struct XwUcgWidget;


impl UcgToTokens for XwUcgWidget {
    fn ucg_to_tokens(&self, tokens: &mut TokenStream, tree: DsTreeRef, ctx: DsContextRef) {
        UcgWidget::ucg_to_tokens(self, tokens, tree, ctx);
    }

    fn ucg_type(&self) -> UcgType {
        todo!()
    }
}

impl UcgWidget for XwUcgWidget {
    fn widget(&self, tokens: &mut proc_macro2::TokenStream, widget_type: &syn::Ident, parent: String) {
        let widget_type = widget_type.to_string();
        let parent = parent.to_string();
        let new_tokens = quote! {
            let widget = #widget_type::new();
            #parent.add_child(widget);
        };
        tokens.extend(new_tokens);
    }
}
