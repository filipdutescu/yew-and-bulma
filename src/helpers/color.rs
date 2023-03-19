use std::fmt::Display;

/// Enum defining the possible text colors, as described in the
/// [Bulma documentation][bd].
///
/// Defines all color values that text can take, as described in the
/// [Bulma color helpers documentation][bd]. Since all of the Bulma classes use
/// the `has-text-*` prefix, this is needed to be included when formatting the
/// color value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
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
/// [bd]: https://bulma.io/documentation/helpers/color-helpers/#text-color
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextColor {
    White,
    Black,
    Light,
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    BlackBis,
    BlackTer,
    GreyDarker,
    GreyDark,
    Grey,
    GreyLight,
    GreyLighter,
    WhiteTer,
    WhiteBis,
}

impl Display for TextColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_name = match self {
            TextColor::White => "white",
            TextColor::Black => "black",
            TextColor::Light => "light",
            TextColor::Dark => "dark",
            TextColor::Primary => "primary",
            TextColor::Link => "link",
            TextColor::Info => "info",
            TextColor::Success => "success",
            TextColor::Warning => "warning",
            TextColor::Danger => "danger",
            TextColor::BlackBis => "black-bis",
            TextColor::BlackTer => "black-ter",
            TextColor::GreyDarker => "grey-darker",
            TextColor::GreyDark => "grey-dark",
            TextColor::Grey => "grey",
            TextColor::GreyLight => "grey-light",
            TextColor::GreyLighter => "grey-lighter",
            TextColor::WhiteTer => "white-ter",
            TextColor::WhiteBis => "white-bis",
        };
        write!(f, "{color_name}")
    }
}

/// Enum defining the possible background colors, as described in the
/// [Bulma documentation][bd].
///
/// Defines all color values that background can take, as described in the
/// [Bulma color helpers documentation][bd]. Since all of the Bulma classes use
/// the `has-background-*` prefix, this is needed to be included when formatting
/// the color value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::BackgroundColor,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the background color set to primary.
/// #[function_component(ColoredBackgroundDiv)]
/// fn colored_text_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_background_color(Some(BackgroundColor::Primary))
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
///     helpers::color::BackgroundColor,
///     utils::constants::HAS_BACKGROUND_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the background color set to primary.
/// #[function_component(ColoredBackgroundDiv)]
/// fn colored_background_div() -> Html {
///     let background_color = BackgroundColor::Primary;
///     let class = classes![format!("{HAS_BACKGROUND_PREFIX}-{background_color}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/color-helpers/#background-color
#[derive(Clone, Debug)]
pub enum BackgroundColor {
    White,
    Black,
    Light,
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    BlackBis,
    BlackTer,
    GreyDarker,
    GreyDark,
    Grey,
    GreyLight,
    GreyLighter,
    WhiteTer,
    WhiteBis,
    PrimaryLight,
    LinkLight,
    InfoLight,
    SuccessLight,
    WarningLight,
    DangerLight,
    PrimaryDark,
    LinkDark,
    InfoDark,
    SuccessDark,
    WarningDark,
    DangerDark,
}

impl Display for BackgroundColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_name = match self {
            BackgroundColor::White => "white",
            BackgroundColor::Black => "black",
            BackgroundColor::Light => "light",
            BackgroundColor::Dark => "dark",
            BackgroundColor::Primary => "primary",
            BackgroundColor::Link => "link",
            BackgroundColor::Info => "info",
            BackgroundColor::Success => "success",
            BackgroundColor::Warning => "warning",
            BackgroundColor::Danger => "danger",
            BackgroundColor::BlackBis => "black-bis",
            BackgroundColor::BlackTer => "black-ter",
            BackgroundColor::GreyDarker => "grey-darker",
            BackgroundColor::GreyDark => "grey-dark",
            BackgroundColor::Grey => "grey",
            BackgroundColor::GreyLight => "grey-light",
            BackgroundColor::GreyLighter => "grey-lighter",
            BackgroundColor::WhiteTer => "white-ter",
            BackgroundColor::WhiteBis => "white-bis",
            BackgroundColor::PrimaryLight => "primary-light",
            BackgroundColor::LinkLight => "link-light",
            BackgroundColor::InfoLight => "info-light",
            BackgroundColor::SuccessLight => "success-light",
            BackgroundColor::WarningLight => "warning-light",
            BackgroundColor::DangerLight => "danger-light",
            BackgroundColor::PrimaryDark => "primary-dark",
            BackgroundColor::LinkDark => "link-dark",
            BackgroundColor::InfoDark => "info-dark",
            BackgroundColor::SuccessDark => "success-dark",
            BackgroundColor::WarningDark => "warning-dark",
            BackgroundColor::DangerDark => "danger-dark",
        };
        write!(f, "{color_name}")
    }
}

/// Enum defining the possible colors, as described in the
/// [Bulma documentation][bd].
///
/// Defines all color values that various elements and components can take, as
/// described throughout the documentation (ie
/// [`crate::elements::button::button`], [`crate::elements::tag::tag`]). Since
/// all of the Bulma classes use the `is-*` prefix, this is needed to be
/// included when formatting the color value. This can be simplified by using
/// the [`crate::utils::class::ClassBuilder`] instead of manually handling
/// creation of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::color::Color,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the color set to primary.
/// #[function_component(ColoredDiv)]
/// fn colored_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_color(Some(Color::Primary))
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
///     helpers::color::Color,
///     utils::constants::IS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the color set to primary.
/// #[function_component(ColoredDiv)]
/// fn colored_div() -> Html {
///     let color = Color::Primary;
///     let class = classes![format!("{IS_PREFIX}-{color}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/customize/variables/
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    Light,
    Dark,
    Text,
    Ghost,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Color::White => "white",
            Color::Black => "black",
            Color::Light => "light",
            Color::Dark => "dark",
            Color::Text => "text",
            Color::Ghost => "ghost",
            Color::Primary => "primary",
            Color::Link => "link",
            Color::Info => "info",
            Color::Success => "success",
            Color::Warning => "warning",
            Color::Danger => "danger",
        };

        write!(f, "{color}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(TextColor::White, "white" ; "white converts to white")]
    #[test_case(TextColor::Black, "black" ; "black converts to black")]
    #[test_case(TextColor::Light, "light" ; "light converts to light")]
    #[test_case(TextColor::Dark, "dark" ; "dark converts to dark")]
    #[test_case(TextColor::Primary, "primary" ; "primary converts to primary")]
    #[test_case(TextColor::Link, "link" ; "link converts to link")]
    #[test_case(TextColor::Info, "info" ; "info converts to info")]
    #[test_case(TextColor::Success, "success" ; "success converts to success")]
    #[test_case(TextColor::Warning, "warning" ; "warning converts to warning")]
    #[test_case(TextColor::Danger, "danger" ; "danger converts to danger")]
    #[test_case(TextColor::BlackBis, "black-bis" ; "black bis converts to black-bis")]
    #[test_case(TextColor::BlackTer, "black-ter" ; "black ter converts to black-ter")]
    #[test_case(TextColor::GreyDarker, "grey-darker" ; "grey darker converts to grey-darker")]
    #[test_case(TextColor::GreyDark, "grey-dark" ; "grey dark converts to grey-dark")]
    #[test_case(TextColor::Grey, "grey" ; "grey converts to grey")]
    #[test_case(TextColor::GreyLight, "grey-light" ; "grey light converts to grey-light")]
    #[test_case(TextColor::GreyLighter, "grey-lighter" ; "grey lighter converts to grey-lighter")]
    #[test_case(TextColor::WhiteTer, "white-ter" ; "white ter converts to white-ter")]
    #[test_case(TextColor::WhiteBis, "white-bis" ; "white bis converts to white-bis")]
    fn text_color_values_to_string(color: TextColor, expected_color: &str) {
        let converted_color = format!("{color}");

        assert_eq!(converted_color, expected_color);
    }

    #[test_case(BackgroundColor::White, "white" ; "white converts to white")]
    #[test_case(BackgroundColor::Black, "black" ; "black converts to black")]
    #[test_case(BackgroundColor::Light, "light" ; "light converts to light")]
    #[test_case(BackgroundColor::Dark, "dark" ; "dark converts to dark")]
    #[test_case(BackgroundColor::Primary, "primary" ; "primary converts to primary")]
    #[test_case(BackgroundColor::Link, "link" ; "link converts to link")]
    #[test_case(BackgroundColor::Info, "info" ; "info converts to info")]
    #[test_case(BackgroundColor::Success, "success" ; "success converts to success")]
    #[test_case(BackgroundColor::Warning, "warning" ; "warning converts to warning")]
    #[test_case(BackgroundColor::Danger, "danger" ; "danger converts to danger")]
    #[test_case(BackgroundColor::BlackBis, "black-bis" ; "black bis converts to black-bis")]
    #[test_case(BackgroundColor::BlackTer, "black-ter" ; "black ter converts to black-ter")]
    #[test_case(BackgroundColor::GreyDarker, "grey-darker" ; "grey darker converts to grey-darker")]
    #[test_case(BackgroundColor::GreyDark, "grey-dark" ; "grey dark converts to grey-dark")]
    #[test_case(BackgroundColor::Grey, "grey" ; "grey converts to grey")]
    #[test_case(BackgroundColor::GreyLight, "grey-light" ; "grey light converts to grey-light")]
    #[test_case(BackgroundColor::GreyLighter, "grey-lighter" ; "grey lighter converts to grey-lighter")]
    #[test_case(BackgroundColor::WhiteTer, "white-ter" ; "white ter converts to white-ter")]
    #[test_case(BackgroundColor::WhiteBis, "white-bis" ; "white bis converts to white-bis")]
    #[test_case(BackgroundColor::PrimaryLight, "primary-light" ; "primary light converts to primary-light")]
    #[test_case(BackgroundColor::LinkLight, "link-light" ; "link light converts to link-light")]
    #[test_case(BackgroundColor::InfoLight, "info-light" ; "info light converts to info-light")]
    #[test_case(BackgroundColor::SuccessLight, "success-light" ; "success light converts to success-light")]
    #[test_case(BackgroundColor::WarningLight, "warning-light" ; "warning light converts to warning-light")]
    #[test_case(BackgroundColor::DangerLight, "danger-light" ; "danger light converts to danger-light")]
    #[test_case(BackgroundColor::PrimaryDark, "primary-dark" ; "primary dark converts to primary-dark")]
    #[test_case(BackgroundColor::LinkDark, "link-dark" ; "link dark converts to link-dark")]
    #[test_case(BackgroundColor::InfoDark, "info-dark" ; "info dark converts to info-dark")]
    #[test_case(BackgroundColor::SuccessDark, "success-dark" ; "success dark converts to success-dark")]
    #[test_case(BackgroundColor::WarningDark, "warning-dark" ; "warning dark converts to warning-dark")]
    #[test_case(BackgroundColor::DangerDark, "danger-dark" ; "danger dark converts to danger-dark")]
    fn background_color_values_to_string(color: BackgroundColor, expected_color: &str) {
        let converted_color = format!("{color}");

        assert_eq!(converted_color, expected_color);
    }

    #[test_case(Color::White, "white" ; "white converts to white")]
    #[test_case(Color::Black, "black" ; "black converts to black")]
    #[test_case(Color::Light, "light" ; "light converts to light")]
    #[test_case(Color::Dark, "dark" ; "dark converts to dark")]
    #[test_case(Color::Text, "text" ; "text converts to text")]
    #[test_case(Color::Ghost, "ghost" ; "ghost converts to ghost")]
    #[test_case(Color::Primary, "primary" ; "primary converts to primary")]
    #[test_case(Color::Link, "link" ; "link converts to link")]
    #[test_case(Color::Info, "info" ; "info converts to info")]
    #[test_case(Color::Success, "success" ; "success converts to success")]
    #[test_case(Color::Warning, "warning" ; "warning converts to warning")]
    #[test_case(Color::Danger, "danger" ; "danger converts to danger")]
    fn color_values_to_string(color: Color, expected_color: &str) {
        let converted_color = format!("{color}");

        assert_eq!(converted_color, expected_color);
    }
}
