pub mod decipher;

use crate::ds_node::DsTreeRef;
use crate::ds_node::ds_attr::DsAttr;
use crate::ds_node::ds_on::DsOn;

/// DsRune — the codegen interface.
/// Implement this trait to generate code from the parsed DSL tree.
/// Each method "inscribes" a node type into the output.
pub trait DsRune {
    /// Inscribe the root node (provides the parent expression).
    fn inscribe_root(&mut self, parent_expr: &syn::Expr);

    /// Inscribe a widget node. `on_handlers` collects every `on EventKind` clause attached to the widget.
    fn inscribe_widget(
        &mut self,
        name: &syn::Ident,
        attrs: &[DsAttr],
        enchants: &[syn::Expr],
        on_handlers: &[DsOn],
        children: &[DsTreeRef],
    );

    /// Inscribe a conditional node. `reactive` is set by a `$` sigil on the
    /// condition (`if $cond` / `if ${ expr }`). `else_branch`, when present, is
    /// either an `Else`-node tree (its children are the else-body) or an
    /// `If`-node tree (`else if` recurses).
    fn inscribe_if(
        &mut self,
        condition: &syn::Expr,
        reactive: bool,
        children: &[DsTreeRef],
        else_branch: Option<&DsTreeRef>,
    );

    /// Inscribe an iteration node.
    fn inscribe_iter(
        &mut self,
        iterable: &syn::Expr,
        variable: &syn::Ident,
        reactive: bool,
        key: Option<&syn::Expr>,
        children: &[DsTreeRef],
    );

    /// Inscribe a niche node. `is_declaration` distinguishes
    /// `@@name` (a template declaring an exposed slot, optionally
    /// with fallback children) from `@name` (a call site filling
    /// the slot).
    fn inscribe_niche(&mut self, name: &syn::Ident, is_declaration: bool, children: &[DsTreeRef]);

    /// Inscribe a match node — `match expr { Pat => { ... } ... }`.
    fn inscribe_match(
        &mut self,
        scrutinee: &syn::Expr,
        reactive: bool,
        arms: &[crate::ds_node::ds_match::DsMatchArm],
    );

    /// Inscribe a `${ ... }` opaque Rust code block. Runes typically
    /// splice the token stream verbatim into the generated code.
    fn inscribe_code_block(&mut self, tokens: &proc_macro2::TokenStream);

    /// Seal the rune — finalize and return the generated TokenStream.
    fn seal(self) -> proc_macro2::TokenStream;
}
