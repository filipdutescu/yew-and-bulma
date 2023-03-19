/// Provides utilities for CSS class manipulation.
///
/// The most important element contained in this module is the
/// [`crate::utils::class::ClassBuilder`] struct, which is used to make it
/// easier to generate the [HTML class attribute][class] value of an element.
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
/// [class]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes#class
pub mod class;
/// Provides various constants in a centralized place.
///
/// Defines constants such as Bulma class name prefixes (ie for `has-text-*`, `is-size-*`,
/// `has-background-*` etc.).
///
/// # Examples
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
pub mod constants;
pub mod size;
