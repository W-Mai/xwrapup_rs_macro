use syn::parse::ParseStream;
syn::custom_keyword!(walk);
syn::custom_keyword!(with);

pub fn is_custom_keyword(input: ParseStream) -> bool {
    input.peek(walk) ||
        input.peek(with)
}
