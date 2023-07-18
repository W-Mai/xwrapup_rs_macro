use crate::ds_node::ds_context::DsContextRef;
use crate::ds_node::DsTreeRef;
use crate::ui_code_gen::ucg_widget::UcgWidget;

mod ucg_widget;
mod xwrapup;

pub enum UcgType {
    Widget(Box<dyn UcgWidget>),
    Attr,
    If,
    Walk,
    None,
}

pub trait UcgToTokens {
    fn ucg_to_tokens(&self, tokens: &mut proc_macro2::TokenStream, tree: DsTreeRef, ctx: DsContextRef);

    fn ucg_type(&self) -> UcgType;
}
