use syn::parse::ParseStream;

pub trait DsNodeIsMe {
    fn is_me(input: ParseStream) -> bool;
}
