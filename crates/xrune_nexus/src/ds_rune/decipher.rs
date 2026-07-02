use super::DsRune;
use crate::ds_node::DsTreeRef;
use crate::ds_node::node_enum::DsNode;

/// Traverse a DsTree and invoke the appropriate DsRune methods.
pub fn decipher(tree: &DsTreeRef, rune: &mut dyn DsRune) {
    let borrowed = tree.borrow();
    match borrowed.get_node() {
        DsNode::Root(expr) => {
            rune.inscribe_root(expr);
            for child in borrowed.get_children() {
                decipher(child, rune);
            }
        }
        DsNode::Widget(widget) => {
            rune.inscribe_widget(
                widget.get_name(),
                &widget.get_attrs().attrs,
                widget.get_enchants(),
                widget.get_on_handlers(),
                borrowed.get_children(),
            );
        }
        DsNode::If(if_node) => {
            rune.inscribe_if(
                if_node.get_condition(),
                if_node.is_reactive(),
                borrowed.get_children(),
                borrowed.get_else_branch(),
            );
        }
        DsNode::Iter(iter_node) => {
            rune.inscribe_iter(
                iter_node.get_iterable(),
                iter_node.get_variable(),
                iter_node.is_reactive(),
                iter_node.get_key(),
                borrowed.get_children(),
            );
        }
        DsNode::Niche(niche_node) => {
            rune.inscribe_niche(niche_node.get_name(), borrowed.get_children());
        }
        DsNode::Match(match_node) => {
            rune.inscribe_match(
                match_node.get_scrutinee(),
                match_node.is_reactive(),
                match_node.get_arms(),
            );
        }
        DsNode::CodeBlock(code) => {
            rune.inscribe_code_block(code.get_tokens());
        }
        DsNode::Else => {
            for child in borrowed.get_children() {
                decipher(child, rune);
            }
        }
    }
}
