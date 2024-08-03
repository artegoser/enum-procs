use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

/// Derive macro generating an impl of the trait `PartialEq` that compare enum only by variant
/// Enum::Variant(value) == Enum::Variant(other_value) => true
#[proc_macro_derive(PartialEqVariant)]
pub fn eq_variant(input: TokenStream) -> TokenStream {
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

/// Derive macro generating an impl of the trait `From` for all types
/// inside tuple variants with one type
#[proc_macro_derive(AutoFrom)]
pub fn auto_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = if let Data::Enum(data) = &input.data {
        &data.variants
    } else {
        panic!("AutoFrom can only be derived for enums");
    };

    let implementations = variants.iter().map(|Variant { ident, fields, .. }| {
        let variant = quote! { #name::#ident };
        match fields {
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return quote! {};
                }

                let field = fields.unnamed.iter().next().unwrap();

                let first = quote! {
                    impl From<#field> for #name {
                        fn from(item: #field) -> Self {
                            #variant(item)
                        }
                    }
                };

                let second = {
                    if field.to_token_stream().to_string() == "String" {
                        quote! {
                            impl From<&str> for #name {
                                fn from(item: &str) -> Self {
                                    #variant(item.to_owned())
                                }
                            }
                        }
                    } else {
                        quote! {}
                    }
                };

                quote! {
                    #first
                    #second
                }
            }
            _ => quote! {},
        }
    });

    let expanded = quote! {
        #(#implementations)*
    };

    TokenStream::from(expanded)
}
