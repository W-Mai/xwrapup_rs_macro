use std::fmt::Debug;
use std::ops::Deref;
use syn::parse::{Parse, ParseStream};

use crate::ds_node::ds_attr::DsAttr;
use crate::ds_node::node_enum::DsNode;
use crate::ds_node::{DsTree, DsTreeRef};

pub struct DsRoot {
    context_attrs: Vec<DsAttr>,
    content: DsTreeRef,
}

impl DsRoot {
    pub fn get_parent(&self) -> syn::Expr {
        // Legacy: find "parent" attr, or return unit expr
        self.context_attrs
            .iter()
            .find(|a| a.name.as_ref().is_some_and(|n| n == "parent"))
            .map(|a| a.value.clone())
            .unwrap_or_else(|| syn::parse_quote!(()))
    }

    pub fn get_content(&self) -> DsTreeRef {
        self.content.clone()
    }

    pub fn get_context_attrs(&self) -> &[DsAttr] {
        &self.context_attrs
    }
}

impl Deref for DsRoot {
    type Target = DsTreeRef;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl Debug for DsRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DsRoot")
            .field("context_attrs", &self.context_attrs.len())
            .field("content", &self.content)
            .finish()
    }
}

impl Parse for DsRoot {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (attrs, parent) = if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;

            let header_span_for_check = input.fork().span();

            let mut attrs = Vec::<DsAttr>::new();
            let params;
            let paren = syn::parenthesized!(params in input);
            while !params.is_empty() {
                attrs.push(params.parse()?);
                if params.peek(syn::Token![,]) {
                    params.parse::<syn::Token![,]>()?;
                }
                if params.peek(syn::Token![:]) {
                    params.parse::<syn::Token![:]>()?;
                }
            }

            if let Some(text) = paren.span.join().source_text()
                && !text.contains('\n')
                && attrs.len() > 1
            {
                return Err(syn::Error::new(
                    header_span_for_check,
                    "root header must be multi-line — put each context attr on its own line, e.g.\n\n    :(\n        attr1: value1\n        attr2: value2\n    :)\n",
                ));
            }

            let parent = attrs
                .iter()
                .find(|attr| attr.name.as_ref().is_some_and(|n| n == "parent"))
                .map(|a| a.value.clone())
                .unwrap_or_else(|| syn::parse_quote!(()));

            (attrs, parent)
        } else {
            (Vec::new(), syn::parse_quote!(()))
        };

        let content = DsTree::parse(input)?.into_ref();
        content.borrow_mut().set_parent(
            DsTree {
                parent: None,
                node: DsNode::Root(parent),
                children: vec![],
                else_branch: None,
            }
            .into_ref(),
        );

        Ok(DsRoot {
            context_attrs: attrs,
            content,
        })
    }
}
