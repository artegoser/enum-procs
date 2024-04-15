use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

/// Compare enum only by variant
/// Enum::Variant(value) == Enum::Variant(other_value) => true
#[proc_macro_derive(PartialEqVariant)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = if let Data::Enum(data) = &input.data {
        &data.variants
    } else {
        panic!("PartialEqVariant can only be derived for enums");
    };

    let variant_checks = variants.iter().map(|Variant { ident, fields, .. }| {
        let variant = quote! { #name::#ident };
        match fields {
            Fields::Unnamed(_) => quote! { (#variant(..), #variant(..)) => true },
            Fields::Unit => quote! { (#variant, #variant) => true },

            _ => panic!(
                "PartialEqVariant can only be derived for enums with unnamed or unit variants"
            ),
        }
    });

    let expanded = quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#variant_checks,)*
                    _ => false,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
