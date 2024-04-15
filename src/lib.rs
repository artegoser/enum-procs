use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};
/// Compare enum only by variant
/// Enum::Variant(value) == Enum::Variant(other_value) => true
#[proc_macro_derive(PartialEqVariant)]
pub fn partial_eq_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    if let Data::Enum(data) = &input.data {
        &data.variants
    } else {
        panic!("PartialEqVariant can only be derived for enums");
    };

    TokenStream::from(quote! {
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                std::mem::discriminant(self) == std::mem::discriminant(other)
            }
        }
    })
}

/// Compare enum except last value
/// Enum::Variant(value, first_value) == Enum::Variant(value, other_second_value) => true
#[proc_macro_derive(PartialEqExceptLast)]
pub fn partial_eq_except_last(input: TokenStream) -> TokenStream {
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
            Fields::Unnamed(fields) => {
                let all_fields = fields.unnamed.iter().rev().skip(1).rev().enumerate();

                let fields1 = all_fields.clone().map(|(i, _)| format_ident!("f_{}", i));
                let fields2 = all_fields.clone().map(|(i, _)| format_ident!("s_{}", i));

                let fields3 = fields1.clone();
                let fields4 = fields2.clone();

                if fields.unnamed.len() > 1 {
                    quote! { (#variant(#(#fields1,)* _), #variant(#(#fields2,)* _)) => #(
                        #fields3 == #fields4
                    )&&* }
                } else {
                    quote! { (#variant(_), #variant(_)) => true }
                }
            }
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
