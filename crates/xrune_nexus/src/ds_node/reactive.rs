use syn::parse::ParseStream;

/// Parse a head expression, returning it with a leading `$` stripped and
/// whether that `$` was present.
pub fn reactive_or_expr(input: ParseStream) -> syn::Result<(syn::Expr, bool)> {
    if input.peek(syn::Token![$]) {
        input.parse::<syn::Token![$]>()?;
        if input.peek(syn::token::Brace) {
            let block: syn::ExprBlock = input.parse()?;
            return Ok((syn::Expr::Block(block), true));
        }
        return Ok((collect_until_brace(input)?, true));
    }
    Ok((collect_until_brace(input)?, false))
}

/// Parse an attribute value, returning it with a leading `$` stripped and
/// whether that `$` was present. Unlike `reactive_or_expr`, the bare case parses
/// a full `syn::Expr` (attr values terminate on `,` / `)`, not a brace body).
pub fn reactive_attr_or_expr(input: ParseStream) -> syn::Result<(syn::Expr, bool)> {
    if input.peek(syn::Token![$]) {
        input.parse::<syn::Token![$]>()?;
        if input.peek(syn::token::Brace) {
            let block: syn::ExprBlock = input.parse()?;
            return Ok((syn::Expr::Block(block), true));
        }
        return Ok((input.parse::<syn::Expr>()?, true));
    }
    Ok((input.parse::<syn::Expr>()?, false))
}

pub fn collect_until_brace(input: ParseStream) -> syn::Result<syn::Expr> {
    let mut tokens = proc_macro2::TokenStream::new();
    while !input.is_empty() && !input.peek(syn::token::Brace) {
        let tt: proc_macro2::TokenTree = input.parse()?;
        tokens.extend(std::iter::once(tt));
    }
    syn::parse2(tokens)
}
