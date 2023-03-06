//! # Yew and Bulma - Bulma CSS components for Yew
//!
//! This crate provides [Bulma CSS][bulma] components made to be used with the
//! [Yew][yew] framework. It aims to make an easy, as intuitive as possible
//! development experience for integrating [Bulma][bulma] into your [Yew][yew]
//! frontends.
//!
//! Generally speaking, it aims to provide a Rust API for ideally* all elements,
//! components, helpers etc. that you would be able to use in CSS/HTML or other
//! frontend frameworks, such as Angular or React.
//!
//! > _* It might not be possible to expose everything in the same manner as
//! with JavaScript, but wherever it is, this crate will try and implement them._
//!
//! ### Supported Targets (for Yew Client-Side Rendering only)
//! - `wasm32-unknown-unknown`
//!
//! # Examples
//!
//! Since it is in the early stages of development, no complete example is made
//! yet.
//!
//! [bulma]: https://bulma.io
//! [yew]: https://yew.rs

pub mod elements;
/// CSS helpers, as described in the [Bulma documentation][bd].
///
/// Contains the [Bulma CSS helpers][bd] implementations for:
///
/// * Color (implemented in the [`crate::helpers::color`] module)
/// * Spacing (implemented in the [`crate::helpers::spacing`] module)
/// * Typography (implemented in the [`crate::helpers::typography`] module)
/// * Visibility (implemented in the [`crate::helpers::visibility`] module)
/// * Flexbox (implemented in the [`crate::helpers::flexbox`] module)
///
/// > _Since the helpers defined in the [Other documentation section][other]
/// only contain individual classes, without much relation or customization to
/// them, those are defined directly into [`crate::utils::constants`]._
///
/// While it is possible to manually format each value, for example using the
/// predefined strings in [`crate::utils::constants`], it is recommended to opt
/// for the [`crate::utils::class::ClassBuilder`] CSS class builder instead.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::TextColor,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the text color set to primary.
/// #[function_component(ColoredTextDiv)]
/// fn colored_text_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_color(Some(TextColor::Primary))
///         .build();
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// It is also possible to use them wihtout the
/// [`crate::utils::class::ClassBuilder`], instead formatting the class names
/// manually, using the constants defined in [`crate::utils::constants`]:
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::TextColor,
///     utils::constants::HAS_TEXT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the text color set to primary.
/// #[function_component(ColoredTextDiv)]
/// fn colored_text_div() -> Html {
///     let text_color = TextColor::Primary;
///     let class = classes![format!("{HAS_TEXT_PREFIX}-{text_color}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/
/// [other]: https://bulma.io/documentation/helpers/other-helpers/
pub mod helpers;
/// Various utilities to make usage of Bulma components and heplers easier in
/// Rust.
pub mod utils;
