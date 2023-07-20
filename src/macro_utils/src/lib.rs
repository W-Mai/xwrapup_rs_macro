extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DsRoot);

    TokenStream::from(input.to_token_stream())
}


#[proc_macro_derive(DsRef)]
pub fn ds_ref_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let ref_name = format_ident!("{}Ref", name);

    let expanded = quote! {
        impl #name {
            pub fn into_ref(self) -> #ref_name {
                #ref_name {
                    inner: Rc::new(RefCell::new(self)),
                }
            }
        }

        #[derive(Debug)]
        pub struct #ref_name {
            inner: Rc<RefCell<#name>>,
        }

        impl #ref_name {
            pub fn borrow(&self) -> std::cell::Ref<#name> {
                self.inner.borrow()
            }

            pub fn borrow_mut(&self) -> std::cell::RefMut<#name> {
                self.inner.borrow_mut()
            }
        }

        impl Clone for #ref_name {
            fn clone(&self) -> Self {
                #ref_name {
                    inner: self.inner.clone(),
                }
            }
        }

    };

    TokenStream::from(expanded)
}
