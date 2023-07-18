use crate::ds_node::ds_context::DsContextRef;
use crate::ds_node::ds_node::{DsNode};
use crate::ds_node::DsTreeRef;
use crate::ui_code_gen::{UcgToTokens, UcgType};

pub trait UcgWidget: UcgToTokens {
    fn ucg_to_tokens(&self, tokens: &mut proc_macro2::TokenStream, tree: DsTreeRef, ctx: DsContextRef) {
        let tree = tree.borrow();
        let node = tree.get_node();
        match node {
            DsNode::Widget(widget) => {
                let widget_type = widget.get_name();
                match &ctx.borrow().parent {
                    Some(parent) => {
                        self.widget(tokens, widget_type, "parent".to_owned());
                    }
                    None => panic!("Expected parent, found None"),
                }
            }
            _ => panic!("Expected widget, found {:?}", node),
        }
    }

    fn ucg_type(&self) -> UcgType;

    fn ucg_is_me(input: syn::parse::ParseStream) -> bool {
        true
    }

    fn widget(&self, tokens: &mut proc_macro2::TokenStream, widget_type: &syn::Ident, parent: String);
}
