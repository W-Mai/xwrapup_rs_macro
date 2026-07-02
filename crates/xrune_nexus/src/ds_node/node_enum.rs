use super::ds_code_block::DsCodeBlock;
use super::ds_if::DsIf;
use super::ds_iter::DsIter;
use super::ds_match::DsMatch;
use super::ds_niche::DsNiche;
use super::ds_traits::DsNodeIsMe;
use super::ds_widget::DsWidget;
use quote::ToTokens;
use std::fmt::Debug;
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub enum DsNodeType {
    Widget,
    If,
    Iter,
    Niche,
    Match,
    CodeBlock,
}

pub enum DsNode {
    Root(syn::Expr),
    Widget(DsWidget),
    If(DsIf),
    Iter(DsIter),
    Niche(DsNiche),
    Match(DsMatch),
    CodeBlock(DsCodeBlock),
    Else,
}

impl Debug for DsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DsNode::Root(expr) => write!(f, "Root({:?})", expr.to_token_stream().to_string()),
            DsNode::Widget(widget) => write!(f, "Widget({widget:?})"),
            DsNode::If(if_node) => write!(f, "If({if_node:?})"),
            DsNode::Iter(iter) => write!(f, "Iter({iter:?})"),
            DsNode::Niche(niche) => write!(f, "Niche({niche:?})"),
            DsNode::Match(match_node) => write!(f, "Match({match_node:?})"),
            DsNode::CodeBlock(code) => write!(f, "{code:?}"),
            DsNode::Else => write!(f, "Else"),
        }
    }
}

impl DsNodeType {
    fn what_type(input: ParseStream) -> syn::Result<DsNodeType> {
        use super::ds_on::DsOn;
        if DsCodeBlock::is_me(input) {
            Ok(DsNodeType::CodeBlock)
        } else if DsNiche::is_me(input) {
            Ok(DsNodeType::Niche)
        } else if DsMatch::is_me(input) {
            Ok(DsNodeType::Match)
        } else if DsWidget::is_me(input) {
            Ok(DsNodeType::Widget)
        } else if DsIf::is_me(input) {
            Ok(DsNodeType::If)
        } else if DsIter::is_me(input) {
            Ok(DsNodeType::Iter)
        } else if DsOn::is_me(input) {
            Err(input.error(
                "`on EventKind` can only follow a widget; \
                 use `Widget() {} on EventKind { ... }` or \
                 `Widget() on EventKind { ... } {}`",
            ))
        } else {
            Err(input.error("expected widget / if / walk / match / @niche / ${ rust code }"))
        }
    }
}

impl Parse for DsNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tree_type = DsNodeType::what_type(input)?;

        let node = match tree_type {
            DsNodeType::Widget => DsNode::Widget(input.parse()?),
            DsNodeType::If => DsNode::If(input.parse()?),
            DsNodeType::Iter => DsNode::Iter(input.parse()?),
            DsNodeType::Niche => DsNode::Niche(input.parse()?),
            DsNodeType::Match => DsNode::Match(input.parse()?),
            DsNodeType::CodeBlock => DsNode::CodeBlock(input.parse()?),
        };

        Ok(node)
    }
}
