use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Variant};

pub fn impl_loose_eq_macro(ast: syn::DeriveInput) -> TokenStream {
    let DeriveInput { ident, data, .. } = ast;

    let syn::Data::Enum(DataEnum { variants, .. }) = data else {
        panic!()
    };

    let match_arms = variants.iter().map(|variant| {
        let Variant {
            ident: var_ident, ..
        } = variant;
        quote! {
            (#ident::#var_ident { .. }, #ident::#var_ident { .. }) => true,
        }
    });

    let gen = quote! {
        impl PartialEq for #ident {
            fn eq(&self, rhs: &Self) -> bool {
                match (self, rhs) {
                    #(#match_arms)*
                    _ => false
                }
            }
        }
    };

    gen.into()
}
