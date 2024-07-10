extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DsRef)]
pub fn ds_ref_derive_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let ref_name = format_ident!("{}Ref", name);

    let expanded = quote! {
        impl #name {
            pub fn into_ref(self) -> #ref_name {
                #ref_name {
                    inner: std::rc::Rc::new(core::cell::RefCell::new(self)),
                }
            }
        }

        #[derive(Debug)]
        pub struct #ref_name {
            inner: std::rc::Rc<core::cell::RefCell<#name>>,
        }
        
        impl Clone for #ref_name {
            fn clone(&self) -> Self {
                #ref_name {
                    inner: self.inner.clone(),
                }
            }
        }
        
        impl core::ops::Deref for #ref_name {
            type Target = std::rc::Rc<std::cell::RefCell<#name>>;
        
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        
        impl core::ops::DerefMut for #ref_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }
    };

    TokenStream::from(expanded)
}
