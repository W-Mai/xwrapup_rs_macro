pub mod xwrapup;

pub enum UcgType {
    Widget,
    Attr,
    If,
    Walk,
    None,
}

pub trait UcgToTokens {
    fn new(tokens: proc_macro2::TokenStream) -> Self;

    fn gen_widget(&self, widget_type: &syn::Ident, parent: String);
}
