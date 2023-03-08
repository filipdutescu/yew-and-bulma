use std::fmt::Display;

/// Enum defining the possible font sizes, as described in the
/// [Bulma documentation][bd].
///
/// Defines all font size values that text can take, as described in the
/// [Bulma typography helpers documentation][bd]. Since all of the Bulma
/// classes use the `is-size-*` prefix, this is needed to be included when
/// formatting the font size value. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextSize,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the font size set to 3.
/// #[function_component(TextSize3Div)]
/// fn text_size_3_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_size(Some(TextSize::Three))
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
///     helpers::typography::TextSize,
///     utils::constants::IS_SIZE_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the font size set to 3.
/// #[function_component(TextSize3Div)]
/// fn text_size_3_div() -> Html {
///     let text_size = TextSize::Three;
///     let class = classes![format!("{IS_SIZE_PREFIX}-{text_size}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#size
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextSize {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Display for TextSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size_value = match self {
            TextSize::One => "1",
            TextSize::Two => "2",
            TextSize::Three => "3",
            TextSize::Four => "4",
            TextSize::Five => "5",
            TextSize::Six => "6",
            TextSize::Seven => "7",
        };

        write!(f, "{size_value}")
    }
}

/// Enum defining the possible text alignments, as described in the
/// [Bulma documentation][bd].
///
/// Defines all text align values that text can take, as described in the
/// [Bulma typography helpers documentation][bd]. Since all of the Bulma
/// classes use the `has-text-*` prefix, this is needed to be included when
/// formatting the text alignment. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextAlignment,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the text center aligned.
/// #[function_component(TextCenteredDiv)]
/// fn text_centered_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_alignment(Some(TextAlignment::Centered))
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
///     helpers::typography::TextAlignment,
///     utils::constants::HAS_TEXT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the text center aligned.
/// #[function_component(TextCenteredDiv)]
/// fn text_centered_div() -> Html {
///     let text_alignment = TextAlignment::Centered;
///     let class = classes![format!("{HAS_TEXT_PREFIX}-{text_alignment}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#alignment
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextAlignment {
    Centered,
    Justified,
    Left,
    Right,
}

impl Display for TextAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alignment_value = match self {
            TextAlignment::Centered => "centered",
            TextAlignment::Justified => "justified",
            TextAlignment::Left => "left",
            TextAlignment::Right => "right",
        };

        write!(f, "{alignment_value}")
    }
}

/// Enum defining the possible text transformations or font styles, as
/// described in the [Bulma documentation][bd].
///
/// Defines all font style values that text can take, as described in the
/// [Bulma typography helpers documentation][bd]. Since all of the Bulma
/// classes use the `is-*` prefix, this is needed to be included when
/// formatting the font style. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextDecoration,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the text italic.
/// #[function_component(ItalicTextDiv)]
/// fn italic_text_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_decoration(TextDecoration::Italic)
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
///     helpers::typography::TextDecoration,
///     utils::constants::IS_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the text italic.
/// #[function_component(ItalicTextDiv)]
/// fn italic_text_div() -> Html {
///     let text_decoration = TextDecoration::Italic;
///     let class = classes![format!("{IS_PREFIX}-{text_decoration}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#text-transformation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextDecoration {
    Capitalized,
    Lowercase,
    Uppercase,
    Italic,
    Underlined,
}

impl Display for TextDecoration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decoration_name = match self {
            TextDecoration::Capitalized => "capitalized",
            TextDecoration::Lowercase => "lowercase",
            TextDecoration::Uppercase => "uppercase",
            TextDecoration::Italic => "italic",
            TextDecoration::Underlined => "underlined",
        };

        write!(f, "{decoration_name}")
    }
}

/// Enum defining the possible text weights, as described in the
/// [Bulma documentation][bd].
///
/// Defines all font weight values that text can take, as described in the
/// [Bulma typography helpers documentation][bd]. Since all of the Bulma
/// classes use the `has-text-weight-*` prefix, this is needed to be included
/// when formatting the font weight. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::TextWeight,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the light font weight.
/// #[function_component(LightTextDiv)]
/// fn light_text_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_text_weight(Some(TextWeight::Light))
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
///     helpers::typography::TextWeight,
///     utils::constants::HAS_TEXT_WEIGHT_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the light font weight.
/// #[function_component(LightTextDiv)]
/// fn light_text_div() -> Html {
///     let text_weight = TextWeight::Light;
///     let class = classes![format!("{HAS_TEXT_WEIGHT_PREFIX}-{text_weight}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#text-weight
#[derive(Clone, Debug, PartialEq)]
pub enum TextWeight {
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
}

impl Display for TextWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let weight_value = match self {
            TextWeight::Light => "light",
            TextWeight::Normal => "normal",
            TextWeight::Medium => "medium",
            TextWeight::SemiBold => "semibold",
            TextWeight::Bold => "bold",
        };

        write!(f, "{weight_value}")
    }
}

/// Enum defining the possible font families, as described in the
/// [Bulma documentation][bd].
///
/// Defines all font family values that text can take, as described in the
/// [Bulma typography helpers documentation][bd]. Since all of the Bulma
/// classes use the `is-family-weight-*` prefix, this is needed to be included
/// when formatting the font family. This can be simplified by using the
/// [`crate::utils::class::ClassBuilder`] instead of manually handling creation
/// of the class strings.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_and_bulma::{
///     helpers::typography::FontFamily,
///     utils::class::ClassBuilder,
/// };
///
/// // Create a `<div>` HTML element that has the code font family.
/// #[function_component(CodeFontDiv)]
/// fn code_font_div() -> Html {
///     let class = ClassBuilder::default()
///         .with_font_family(Some(FontFamily::Code))
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
///     helpers::typography::FontFamily,
///     utils::constants::IS_FONT_FAMILY_PREFIX,
/// };
///
/// // Create a `<div>` HTML element that has the code font family.
/// #[function_component(CodeFontDiv)]
/// fn code_font_div() -> Html {
///     let font_family = FontFamily::Code;
///     let class = classes![format!("{IS_FONT_FAMILY_PREFIX}-{font_family}")];
///     html!{
///         <div class={class}>{ "Lorem ispum..." }</div>
///     }
/// }
/// ```
///
/// [bd]: https://bulma.io/documentation/helpers/typography-helpers/#font-family
#[derive(Clone, Debug, PartialEq)]
pub enum FontFamily {
    SansSerif,
    Monospace,
    Primary,
    Secondary,
    Code,
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let font_family = match self {
            FontFamily::SansSerif => "sans-serif",
            FontFamily::Monospace => "monospace",
            FontFamily::Primary => "primary",
            FontFamily::Secondary => "secondary",
            FontFamily::Code => "code",
        };

        write!(f, "{font_family}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(TextSize::One, "1" ; "one converts to 1")]
    #[test_case(TextSize::Two, "2" ; "two converts to 2")]
    #[test_case(TextSize::Three, "3" ; "three converts to 3")]
    #[test_case(TextSize::Four, "4" ; "four converts to 4")]
    #[test_case(TextSize::Five, "5" ; "five converts to 5")]
    #[test_case(TextSize::Six, "6" ; "six converts to 6")]
    #[test_case(TextSize::Seven, "7" ; "seven converts to 7")]
    fn text_size_values_to_string(size: TextSize, expected_size: &str) {
        let converted_size = format!("{size}");

        assert_eq!(converted_size, expected_size);
    }

    #[test_case(TextDecoration::Capitalized, "capitalized" ; "capitalized converts to capitalized")]
    #[test_case(TextDecoration::Lowercase, "lowercase" ; "lowercase converts to lowercase")]
    #[test_case(TextDecoration::Uppercase, "uppercase" ; "uppercase converts to uppercase")]
    #[test_case(TextDecoration::Italic, "italic" ; "italic converts to italic")]
    #[test_case(TextDecoration::Underlined, "underlined" ; "underlined converts to underlined")]
    fn text_decoration_values_to_string(text_decoration: TextDecoration, expected_transform: &str) {
        let converted_transform = format!("{text_decoration}");

        assert_eq!(converted_transform, expected_transform);
    }

    #[test_case(TextWeight::Light, "light" ; "light converts to light")]
    #[test_case(TextWeight::Normal, "normal" ; "normal converts to normal")]
    #[test_case(TextWeight::Medium, "medium" ; "medium converts to medium")]
    #[test_case(TextWeight::SemiBold, "semibold" ; "semi bold converts to semibold")]
    #[test_case(TextWeight::Bold, "bold" ; "bold converts to bold")]
    fn text_weight_values_to_string(text_weight: TextWeight, expected_weight: &str) {
        let converted_weight = format!("{text_weight}");

        assert_eq!(converted_weight, expected_weight);
    }

    #[test_case(FontFamily::SansSerif, "sans-serif" ; "sans serif converts to sans-serif")]
    #[test_case(FontFamily::Monospace, "monospace" ; "monospace converts to monospace")]
    #[test_case(FontFamily::Primary, "primary" ; "primary converts to primary")]
    #[test_case(FontFamily::Secondary, "secondary" ; "secondary converts to secondary")]
    #[test_case(FontFamily::Code, "code" ; "code converts to code")]
    fn font_family_values_to_string(font_family: FontFamily, expected_font_family: &str) {
        let converted_font_family = format!("{font_family}");

        assert_eq!(converted_font_family, expected_font_family);
    }
}
