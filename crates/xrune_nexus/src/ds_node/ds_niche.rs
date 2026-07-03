use super::ds_traits::DsNodeIsMe;
use std::fmt::Debug;
use syn::parse::{Parse, ParseStream};

/// A niche node written either `@name` (fill) or `@@name` (declare).
///
/// The DSL is deliberately symmetric: template-authoring macros
/// (mirui's `mold!`, others down the line) use `@@name` to declare a
/// slot they expose; the call-site `ui!` uses `@name` to fill that
/// slot with children. Nested mold composition disambiguates without
/// runtime inspection because the two sigils are lexically distinct.
pub struct DsNiche {
    name: syn::Ident,
    is_declaration: bool,
}

impl DsNiche {
    pub fn get_name(&self) -> &syn::Ident {
        &self.name
    }

    /// `true` when the source used `@@name`; `false` for `@name`.
    pub fn is_declaration(&self) -> bool {
        self.is_declaration
    }
}

impl Debug for DsNiche {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_declaration {
            write!(f, "NicheDecl(@@{})", self.name)
        } else {
            write!(f, "Niche(@{})", self.name)
        }
    }
}

impl Parse for DsNiche {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![@]>()?;
        let is_declaration = if input.peek(syn::Token![@]) {
            input.parse::<syn::Token![@]>()?;
            true
        } else {
            false
        };
        let name = input.parse::<syn::Ident>()?;
        Ok(DsNiche {
            name,
            is_declaration,
        })
    }
}

impl DsNodeIsMe for DsNiche {
    fn is_me(input: ParseStream) -> bool {
        input.peek(syn::Token![@])
    }
}
