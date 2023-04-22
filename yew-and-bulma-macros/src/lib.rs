//! # Procedural macros for [Yew and Bulma][yew-and-bulma]
//!
//! This crate provides various [procedural macros][proc-macro] for the
//! [Yew and Bulma][yew-and-bulma] crate. Most importantly, the
//! [`macro@crate::base_component_properties`] macro, which adds various HTML
//! attributes, such as `id`, `class` or event attributes like `onclick`.
//!
//! ### Supported Targets (for Yew Client-Side Rendering only)
//! - `wasm32-unknown-unknown`
//!
//! # Examples
//!
//! ```rust
//! use yew_and_bulma_macros::base_component_properties;
//!
//! // This will add the `id`, `class` and all standard event HTML attributes.
//! #[base_component_properties]
//! // #[derive(Properties, PartialEq)] // From yew
//! struct MyProperties;
//! ```
//!
//! [yew-and-bulma]: https://crates.io/crates/yew-and-bulma
//! [proc-macro]: https://doc.rust-lang.org/reference/procedural-macros.html
#![cfg_attr(nightly_error_messages, feature(rustc_attrs))]
#![forbid(unsafe_code)]

/// Provides all HTML attributes which should be added to properties.
///
/// Defines helpers and provides definitions for all HTML attributes that
/// should be found on [Yew component properties][yew].
///
/// [yew]: https://yew.rs/docs/concepts/function-components/properties
mod attributes;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

use crate::attributes::BaseAttributes;

/// Adds fields for the `id`, `class` and [all standard event][events] HTML
/// attributes.
///
/// # Examples
///
/// ```rust
/// use yew_and_bulma_macros::base_component_properties;
///
/// // This will add the `id`, `class` and all standard event HTML attributes.
/// #[base_component_properties]
/// // #[derive(Properties, PartialEq)] // From yew
/// struct MyProperties;
/// ```
///
/// [events]: https://developer.mozilla.org/en-US/docs/Web/API/Element#events
#[proc_macro_attribute]
pub fn base_component_properties(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident.clone();
    let mut generics = syn::Generics::default();
    generics.lt_token = input.generics.lt_token;
    generics.params = input.generics.params.clone();
    generics.gt_token = input.generics.gt_token;
    let where_clause = input.generics.params.clone();
    let mut struct_data = match input.data.clone() {
        syn::Data::Struct(struct_data) => struct_data,
        _ => {
            let ident = input.ident.span();
            return quote_spanned!(ident => syn::Error::new(ident, "`BaseProperties` must be used on structs.")
                .to_compile_error()).into();
        }
    };

    let base_attribs = BaseAttributes::default().attributes();
    let base_attrib_idents: Vec<_> = base_attribs
        .iter()
        .filter_map(|f| f.ident.clone())
        .collect();
    let expanded = match &mut struct_data.fields {
        syn::Fields::Named(fields) => {
            for attr in base_attribs {
                fields.named.push(attr);
            }

            let struct_data = DeriveInput {
                data: syn::Data::Struct(struct_data),
                ..input
            };

            quote! {
                #struct_data
            }
        }
        _ => quote! { #input },
    };

    let expanded = if ident == "BaseComponentProperties" {
        expanded
    } else {
        quote! {
            #expanded

            impl #generics From<#ident #generics> for crate::utils::BaseComponentProperties #where_clause {
                fn from(value: #ident #generics) -> Self {
                    crate::utils::BaseComponentProperties {
                        tag: yew::AttrValue::default(),
                        children: yew::Children::default(),
                        #(#base_attrib_idents: value.#base_attrib_idents),*
                    }
                }
            }

            impl #generics From<&#ident #generics> for crate::utils::BaseComponentProperties #where_clause {
                fn from(value: &#ident #generics) -> Self {
                    crate::utils::BaseComponentProperties {
                        tag: yew::AttrValue::default(),
                        children: yew::Children::default(),
                        #(#base_attrib_idents: value.#base_attrib_idents.clone()),*
                    }
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(TypedChildren)]
pub fn typed_children(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data_enum = match &input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            let ident = input.ident.span();
            return quote_spanned!(ident => syn::Error::new(ident, "`TypedChildren` must be used on enums.")
                .to_compile_error()).into();
        }
    };

    let ident = &input.ident;
    let mut variants = Vec::with_capacity(data_enum.variants.iter().count());
    let from_impls: Vec<_> = data_enum
        .variants
        .iter()
        .map(|v| {
            let var_ident = &v.ident;
            let field = match &v.fields {
                syn::Fields::Unnamed(fields) => fields.unnamed.first(),
                _ => {
                    return quote! {};
                }
            };
            let field = match field {
                Some(field) => field,
                None => {
                    return quote! {};
                }
            };
            variants.push(quote! {#ident::#var_ident});

            quote! {
                impl From<#field> for #ident {
                    fn from(value: #field) -> Self {
                        #ident::#var_ident(value)
                    }
                }
            }
        })
        .collect();

    quote! {
        // #input

        #(#from_impls)
        *

        #[allow(clippy::from_over_into)]
        impl Into<yew::Html> for #ident {
            fn into(self) -> yew::Html {
                match self {
                    #(#variants(v) => v.into()),*
                }
            }
        }
    }
    .into()
}
