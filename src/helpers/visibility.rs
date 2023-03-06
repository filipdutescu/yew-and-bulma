use std::fmt;

/// Enum defining the possible display values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the display values for which Bulma provides helpers, as described
/// in the [Bulma documentation][bd]. Since all of the Bulma classes use the
/// `is-*` prefix, this is needed to be included when formatting the display
/// value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::visibility::Display,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the display set to flex.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_display(Some(Display::Flex))
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
///     helpers::visibility::Display,
///     utils::constants::IS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the display set to flex.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     let display = Display::Flex;
///     let class = classes![format!("{IS_PREFIX}-{display}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/visibility-helpers/#show
/// [`display`]: https://developer.mozilla.org/en-US/docs/Web/CSS/display
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Display {
    Block,
    Flex,
    Inline,
    InlineBlock,
    InlineFlex,
    Hidden,
    Invisible,
    ScreenReaderOnly,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_value = match self {
            Display::Block => "block",
            Display::Flex => "flex",
            Display::Inline => "inline",
            Display::InlineBlock => "inline-block",
            Display::InlineFlex => "inline-flex",
            Display::Hidden => "hidden",
            Display::Invisible => "invisible",
            Display::ScreenReaderOnly => "sr-only",
        };

        write!(f, "{display_value}")
    }
}

/// Enum defining the possible viewport values, as described in the
/// [Bulma documentation][bd].
///
/// Defines the viewport values which affect other Bulma helpers, such as
/// [`crate::helpers::visibility::Display`], as described in the
/// [Bulma documentation][bd]. Since all of the Bulma classes use the
/// `is-*-*` template, this is needed to be included when formatting the display
/// value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::visibility::{Display, Viewport},
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the display set to flex for the
/// //tablet viewport.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_viewport_display(Display::Flex, Viewport::Tablet)
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
///     helpers::visibility::{Display, Viewport},
///     utils::constants::IS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the display set to flex for the
/// //tablet viewport.
/// #[function_component(FlexDiv)]
/// fn flex_div() -> Html {
///     let display = Display::Flex;
///     let viewport = Viewport::Tablet;
///     let class = classes![format!("{IS_PREFIX}-{display}-{viewport}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/visibility-helpers/#show
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Viewport {
    Mobile,
    Touch,
    TabletOnly,
    Tablet,
    DesktopOnly,
    Desktop,
    WidescreenOnly,
    Widescreen,
    FullHD,
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let viewport_name = match self {
            Viewport::Mobile => "mobile",
            Viewport::Touch => "touch",
            Viewport::TabletOnly => "tablet-only",
            Viewport::Tablet => "tablet",
            Viewport::DesktopOnly => "desktop-only",
            Viewport::Desktop => "desktop",
            Viewport::WidescreenOnly => "widescreen-only",
            Viewport::Widescreen => "widescreen",
            Viewport::FullHD => "fullhd",
        };

        write!(f, "{viewport_name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Display::Block, "block" ; "block converts to block")]
    #[test_case(Display::Inline, "inline" ; "inline converts to inline")]
    #[test_case(Display::InlineBlock, "inline-block" ; "inline block converts to inline-block")]
    #[test_case(Display::InlineFlex, "inline-flex" ; "inline flex converts to inline-flex")]
    #[test_case(Display::Hidden, "hidden" ; "hidden converts to hidden")]
    #[test_case(Display::Invisible, "invisible" ; "invisible converts to invisible")]
    #[test_case(
        Display::ScreenReaderOnly,
        "sr-only" ;
        "screen reader only converts to sr-only"
    )]
    fn display_values_to_string(given_display: Display, expected_display: &str) {
        let converted_display = format!("{given_display}");

        assert_eq!(converted_display, expected_display);
    }

    #[test_case(Viewport::Mobile, "mobile" ; "mobile converts to mobile")]
    #[test_case(Viewport::Touch, "touch" ; "touch converts to touch")]
    #[test_case(Viewport::TabletOnly, "tablet-only" ; "tablet only converts to tablet-only")]
    #[test_case(Viewport::Tablet, "tablet" ; "tablet converts to tablet")]
    #[test_case(Viewport::DesktopOnly, "desktop-only" ; "desktop only converts to desktop-only")]
    #[test_case(Viewport::Desktop, "desktop" ; "desktop converts to desktop")]
    #[test_case(Viewport::WidescreenOnly, "widescreen-only" ; "widescreen only converts to widescreen-only")]
    #[test_case(Viewport::Widescreen, "widescreen" ; "widescreen converts to widescreen")]
    #[test_case(Viewport::FullHD, "fullhd" ; "full hd converts to fullhd")]
    fn viewport_values_to_string(viewport: Viewport, expected_viewport: &str) {
        let converted_viewport = format!("{viewport}");

        assert_eq!(converted_viewport, expected_viewport);
    }
}
