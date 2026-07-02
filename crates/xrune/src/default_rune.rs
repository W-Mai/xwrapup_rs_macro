use proc_macro2::TokenStream;
use quote::quote;

use xrune_nexus::ds_node::DsTreeRef;
use xrune_nexus::ds_node::ds_attr::DsAttr;
use xrune_nexus::ds_rune::DsRune;
use xrune_nexus::ds_rune::decipher::decipher;

/// DefaultRune — generates println-based debug output (default debug output).
pub struct DefaultRune {
    tokens: TokenStream,
    parent_name: String,
}

impl Default for DefaultRune {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRune {
    pub fn new() -> Self {
        Self {
            tokens: TokenStream::new(),
            parent_name: String::new(),
        }
    }
}

impl DsRune for DefaultRune {
    fn inscribe_root(&mut self, parent_expr: &syn::Expr) {
        let parent_string = "parent".to_string();
        self.tokens.extend(quote! {
            println!("let {} = {:?}", #parent_string, #parent_expr);
        });
        self.parent_name = "parent".to_string();
    }

    fn inscribe_widget(
        &mut self,
        name: &syn::Ident,
        attrs: &[DsAttr],
        _enchants: &[syn::Expr],
        on_handlers: &[xrune_nexus::ds_node::ds_on::DsOn],
        children: &[DsTreeRef],
    ) {
        let name_string = name.to_string();
        let parent_name = &self.parent_name;

        self.tokens.extend(quote! {
            println!("let {} = obj::new({})", #name_string, #parent_name);
        });

        for attr in attrs {
            let attr_name = match &attr.name {
                Some(n) => n.to_string(),
                None => "<positional>".to_string(),
            };
            let attr_value = &attr.value;
            self.tokens.extend(quote! {
                println!("{}.set_{}({:?})", #name_string, #attr_name, #attr_value);
            });
        }

        for on in on_handlers {
            let event_name = on.get_name().to_string();
            let arity = on.get_args().len();
            self.tokens.extend(quote! {
                println!("{}.on_{}({} args)", #name_string, #event_name, #arity);
            });
        }

        let prev_parent = self.parent_name.clone();
        self.parent_name = name_string;
        for child in children {
            decipher(child, self);
        }
        self.parent_name = prev_parent;
    }

    fn inscribe_if(
        &mut self,
        condition: &syn::Expr,
        _reactive: bool,
        children: &[DsTreeRef],
        else_branch: Option<&DsTreeRef>,
    ) {
        let con = quote!(#condition).to_string();
        self.tokens.extend(quote! {
            println!("if {} {{", #con);
        });

        for child in children {
            decipher(child, self);
        }

        self.tokens.extend(quote! {
            println!("}}");
        });

        if let Some(branch) = else_branch {
            decipher(branch, self);
        }
    }

    fn inscribe_iter(
        &mut self,
        iterable: &syn::Expr,
        variable: &syn::Ident,
        _reactive: bool,
        _key: Option<&syn::Expr>,
        children: &[DsTreeRef],
    ) {
        let iterable_str = quote!(#iterable).to_string();
        let variable_str = variable.to_string();

        self.tokens.extend(quote! {
            println!("for {} in {} {{", #variable_str, #iterable_str);
        });

        for child in children {
            decipher(child, self);
        }

        self.tokens.extend(quote! {
            println!("}}");
        });
    }

    fn inscribe_niche(&mut self, name: &syn::Ident, children: &[DsTreeRef]) {
        let name_str = name.to_string();
        self.tokens.extend(quote! {
            println!("@{} {{", #name_str);
        });
        for child in children {
            decipher(child, self);
        }
        self.tokens.extend(quote! {
            println!("}}");
        });
    }

    fn inscribe_code_block(&mut self, tokens: &proc_macro2::TokenStream) {
        let repr = tokens.to_string();
        self.tokens.extend(quote! {
            println!("${{ {} }}", #repr);
        });
    }

    fn inscribe_match(
        &mut self,
        scrutinee: &syn::Expr,
        _reactive: bool,
        arms: &[xrune_nexus::ds_node::ds_match::DsMatchArm],
    ) {
        let scrutinee_str = quote!(#scrutinee).to_string();
        self.tokens.extend(quote! {
            println!("match {} {{", #scrutinee_str);
        });
        for arm in arms {
            let pat = arm.get_pat();
            let pat_str = quote!(#pat).to_string();
            self.tokens.extend(quote! {
                println!("  {} => {{", #pat_str);
            });
            for child in arm.get_children() {
                decipher(child, self);
            }
            self.tokens.extend(quote! {
                println!("  }}");
            });
        }
        self.tokens.extend(quote! {
            println!("}}");
        });
    }

    fn seal(self) -> TokenStream {
        self.tokens
    }
}
