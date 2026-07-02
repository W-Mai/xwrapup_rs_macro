use super::ds_traits::DsNodeIsMe;
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::parse::{Parse, ParseStream};

/// A `${ ... }` opaque Rust escape hatch inside the DSL body. The
/// parser captures the token stream verbatim; downstream runes splice
/// it into the generated code without interpreting it.
///
/// `source_text` is the original brace-body text (whitespace and all)
/// when available — codegen runes ignore it, but xrune-fmt uses it to
/// round-trip multi-line blocks without collapsing whitespace or
/// re-quoting operators through TokenStream::to_string.
pub struct DsCodeBlock {
    tokens: TokenStream,
    source_text: Option<String>,
}

impl DsCodeBlock {
    pub fn get_tokens(&self) -> &TokenStream {
        &self.tokens
    }

    pub fn get_source_text(&self) -> Option<&str> {
        self.source_text.as_deref()
    }
}

impl Debug for DsCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeBlock({})", self.tokens)
    }
}

impl Parse for DsCodeBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![$]>()?;
        let content;
        let brace = syn::braced!(content in input);
        let tokens: TokenStream = content.parse()?;
        let source_text = brace.span.join().source_text();
        Ok(DsCodeBlock {
            tokens,
            source_text,
        })
    }
}

impl DsNodeIsMe for DsCodeBlock {
    fn is_me(input: ParseStream) -> bool {
        input.peek(syn::Token![$]) && input.peek2(syn::token::Brace)
    }
}
