use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput};

/// Utility macro for creating an enum with the same, but empty variants of a given enum.
#[proc_macro_derive(TokenKind)]
pub fn derive_token_kind(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_token_kind_macro(ast)
}

fn impl_token_kind_macro(ast: DeriveInput) -> TokenStream {
    let DeriveInput { ident, data, .. } = ast;

    let syn::Data::Enum(DataEnum { variants, .. }) = data else {
        panic!("#[derive(TokenKind)] only works on enums")
    };

    let enum_name = ident.clone();

    let variants = variants.into_iter().map(|variant| variant.ident);

    let token_kinds = variants.clone().map(|ident| {
        quote! {
            #ident,
        }
    });

    let token_kind_token_matches = variants.clone().map(|ident| {
        quote! {
            (TokenKind::#ident, #enum_name::#ident { .. }) => true,
        }
    });

    let token_token_kind_matches = variants.clone().map(|ident| {
        quote! {
            (#enum_name::#ident { .. }, TokenKind::#ident) => true,
        }
    });

    let gen = quote! {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum TokenKind {
            #(#token_kinds)*
        }

        impl PartialEq<#enum_name> for TokenKind {
            fn eq(&self, other: &#enum_name) -> bool {
                match (self, other) {
                    #(#token_kind_token_matches)*
                    _ => false
                }
            }
        }

        impl PartialEq<TokenKind> for #enum_name {
            fn eq(&self, other: &TokenKind) -> bool {
                match (self, other) {
                    #(#token_token_kind_matches)*
                    _ => false
                }
            }
        }

    };

    gen.into()
}
