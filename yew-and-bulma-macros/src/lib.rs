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

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
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

    let mut struct_data = match input.data.clone() {
        syn::Data::Struct(struct_data) => struct_data,
        _ => panic!("`BaseProperties` must be used on structs."),
    };

    let expanded = match &mut struct_data.fields {
        syn::Fields::Named(fields) => {
            for attr in BaseAttributes::default().attributes() {
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

    expanded.into()
}
