use std::fmt::Display;

/// Enum defining the most commonly found element sizes, as found throughout
/// the [Bulma documentation][bd].
///
/// Defines the most commonly found sizes that elements can take, as described
/// in the [Bulma documentation][bd], such as for
/// [`crate::elements::tag::TagProperties::size`] or
/// [`crate::elements::button::ButtonProperties::size`]. Since all of the Bulma
/// classes use the `are-*` or `is-*` prefixes, this is needed to be included
/// when formatting the size value.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     utils::{constants::IS_PREFIX, size::Size},
/// };
///
/// // Create a `<div>` HTML element that has the size set to large.
/// #[function_component(LargeDiv)]
/// fn large_div() -> Html {
///     let size = Size::Large;
///     let class = classes![format!("{IS_PREFIX}-{size}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/
#[derive(PartialEq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = match self {
            Size::Small => "small",
            Size::Normal => "normal",
            Size::Medium => "medium",
            Size::Large => "large",
        };

        write!(f, "{size}")
    }
}
