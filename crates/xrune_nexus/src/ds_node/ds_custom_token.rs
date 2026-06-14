use syn::parse::ParseStream;
syn::custom_keyword!(walk);
syn::custom_keyword!(with);
syn::custom_keyword!(on);
syn::custom_keyword!(elif);
syn::custom_keyword!(by);

pub fn is_custom_keyword(input: ParseStream) -> bool {
    input.peek(walk) || input.peek(with) || input.peek(on) || input.peek(elif) || input.peek(by)
}
